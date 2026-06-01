import { and, desc, eq, isNull, ne, sql } from "drizzle-orm";
import { getDb } from "$lib/db";
import { folder, recast, share, tag } from "$lib/db/schema";
import { QUOTA } from "$lib/db/schema/usage";
import type { PageServerLoad } from "./$types";

// Archived rows only ever belong to Free workspaces (Pro/Enterprise never
// auto-archive), so the Free hard-delete window is the correct countdown for
// every archived recast we surface.
const HARD_DELETE_DAYS = QUOTA.free.hardDeleteAfterArchiveDays ?? 16;
const DAY_MS = 24 * 60 * 60 * 1000;

/**
 * Full library loader. Larger limit than the home page.
 *
 * Returns the active recast list (each with its `folderId` + `tags` id
 * array), plus the workspace's folder tree and tag set, so the library can
 * render organization without extra client round-trips on first paint.
 *
 * Also returns the workspace's `archived` recasts — blobless rows the expiry
 * job parked after 14 days with no views. These power the "Archived" tab,
 * which lets the owner delete them outright or re-upload from desktop before
 * the hard-delete sweep purges them at `archivedAt + 16d`.
 */
export const load: PageServerLoad = async ({ parent }) => {
	const { activeOrganization } = await parent();
	const db = getDb();
	const workspaceId = activeOrganization.id;

	const [rows, archivedRows, folders, tags] = await Promise.all([
		db
			.select({
				id: recast.id,
				title: recast.title,
				durationSec: recast.durationSec,
				sizeBytes: recast.sizeBytes,
				source: recast.source,
				provider: recast.provider,
				status: recast.status,
				folderId: recast.folderId,
				videoUrl: recast.videoUrl,
				posterUrl: recast.posterUrl,
				createdAt: recast.createdAt,
				views: sql<number>`COALESCE((
					SELECT SUM(${share.viewsCount})
					FROM ${share}
					WHERE ${share.recastId} = ${recast.id}
				), 0)`,
				latestShareSlug: sql<string | null>`(
					SELECT ${share.slug}
					FROM ${share}
					WHERE ${share.recastId} = ${recast.id}
					ORDER BY ${share.createdAt} DESC
					LIMIT 1
				)`,
				// Tag id array per recast — resolved against the `tags` list
				// below in the UI. `[]` when untagged.
				tags: sql<string[]>`COALESCE((
					SELECT json_agg(rt.tag_id)
					FROM recast_tag rt
					WHERE rt.recast_id = ${recast.id}
				), '[]'::json)`,
			})
			.from(recast)
			.where(and(eq(recast.workspaceId, workspaceId), ne(recast.status, "archived")))
			.orderBy(desc(recast.createdAt))
			.limit(200),
		db
			.select({
				id: recast.id,
				title: recast.title,
				durationSec: recast.durationSec,
				sizeBytes: recast.sizeBytes,
				posterUrl: recast.posterUrl,
				archivedAt: recast.archivedAt,
				createdAt: recast.createdAt,
			})
			.from(recast)
			.where(
				and(
					eq(recast.workspaceId, workspaceId),
					eq(recast.status, "archived"),
					// Rows the hard-delete sweep has tombstoned are effectively gone.
					isNull(recast.deletedAt),
				),
			)
			.orderBy(desc(recast.archivedAt))
			.limit(100),
		db
			.select({
				id: folder.id,
				parentId: folder.parentId,
				name: folder.name,
				color: folder.color,
				path: folder.path,
			})
			.from(folder)
			.where(eq(folder.workspaceId, workspaceId)),
		db
			.select({ id: tag.id, name: tag.name, color: tag.color })
			.from(tag)
			.where(eq(tag.workspaceId, workspaceId)),
	]);

	return {
		workspaceId,
		recasts: rows.map((r) => ({
			...r,
			sizeBytes: Number(r.sizeBytes),
			views: Number(r.views ?? 0),
			createdAt: r.createdAt.getTime(),
			tags: r.tags ?? [],
		})),
		archived: archivedRows.map((r) => {
			const archivedMs = (r.archivedAt ?? r.createdAt).getTime();
			return {
				id: r.id,
				title: r.title,
				durationSec: r.durationSec,
				sizeBytes: Number(r.sizeBytes),
				posterUrl: r.posterUrl,
				archivedAt: archivedMs,
				// When the hard-delete sweep will purge the row + poster for good.
				deletesAt: archivedMs + HARD_DELETE_DAYS * DAY_MS,
			};
		}),
		folders,
		tags,
	};
};

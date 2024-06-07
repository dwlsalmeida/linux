// SPDX-License-Identifier: GPL-2.0
// SPDX-FileCopyrightText: Copyright Collabora 2024

#include <drm/drm_gem.h>

struct PanthorDumpArgs {
	/**
   * The slot for the job
   */
	s32 slot;
	/**
   * The active buffer objects
   */
	struct drm_gem_object *bos;
	/**
   * The number of active buffer objects
   */
	size_t bo_count;
	/**
   * The base address of the registers to use when reading.
   */
	void *reg_base_addr;
};

/**
 * Dumps the current state of the GPU to a file
 *
 * # Safety
 *
 * All fields of `DumpArgs` must be valid.
 */
#ifdef CONFIG_DRM_PANTHOR_RS
int panthor_core_dump(const struct PanthorDumpArgs *args);
#else
inline int panthor_core_dump(const struct PanthorDumpArgs *args)
{
	return 0;
}
#endif

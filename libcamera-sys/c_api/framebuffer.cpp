#include "framebuffer.h"

#include <vector>

extern "C" {

// --- libcamera_frame_metadata_t ---
libcamera_frame_metadata_status_t libcamera_frame_metadata_status(const libcamera_frame_metadata_t *metadata) {
    return metadata->status;
}

unsigned int libcamera_frame_metadata_sequence(const libcamera_frame_metadata_t *metadata) {
    return metadata->sequence;
}

uint64_t libcamera_frame_metadata_timestamp(const libcamera_frame_metadata_t *metadata) {
    return metadata->timestamp;
}

libcamera_frame_metadata_planes_t *libcamera_frame_metadata_planes(libcamera_frame_metadata_t *metadata) {
    return new libcamera::Span<libcamera::FrameMetadata::Plane>(std::move(metadata->planes()));
}

// --- libcamera_frame_metadata_planes_t ---
void libcamera_frame_metadata_planes_destroy(libcamera_frame_metadata_planes_t *planes) {
    delete planes;
}

size_t libcamera_frame_metadata_planes_size(const libcamera_frame_metadata_planes_t *planes) {
    return planes->size();
}

libcamera_frame_metadata_plane_t *libcamera_frame_metadata_planes_at(libcamera_frame_metadata_planes_t *planes, size_t index) {
    return &planes->data()[index];
}

// --- libcamera_framebuffer_t ---
struct libcamera_framebuffer_planes {
    std::vector<libcamera::FrameBuffer::Plane> planes;
};

libcamera_framebuffer_planes_t *libcamera_framebuffer_planes(const libcamera_framebuffer_t *framebuffer) {
    auto wrapper = new libcamera_framebuffer_planes_t();
    const auto planes = framebuffer->planes();
    wrapper->planes.assign(planes.begin(), planes.end());
    return wrapper;
}

const libcamera_frame_metadata_t *libcamera_framebuffer_metadata(const libcamera_framebuffer_t *framebuffer) {
    return &framebuffer->metadata();
}

uint64_t libcamera_framebuffer_cookie(const libcamera_framebuffer_t *framebuffer) {
    return framebuffer->cookie();
}

int libcamera_framebuffer_release_fence(libcamera_framebuffer_t *framebuffer) {
    auto fence = framebuffer->releaseFence();
    if (!fence)
        return -1;
    return fence->fd().get();
}

// --- libcamera_framebuffer_plane_t ---
int libcamera_framebuffer_plane_fd(libcamera_framebuffer_plane_t *plane) {
    return plane->fd.get();
}

size_t libcamera_framebuffer_plane_offset(const libcamera_framebuffer_plane_t *plane) {
    return plane->offset;
}

bool libcamera_framebuffer_plane_offset_valid(const libcamera_framebuffer_plane_t *plane) {
    return plane->offset != plane->kInvalidOffset;
}

size_t libcamera_framebuffer_plane_length(const libcamera_framebuffer_plane_t *plane) {
    return plane->length;
}

// --- libcamera_framebuffer_planes_t ---
void libcamera_framebuffer_planes_destroy(libcamera_framebuffer_planes_t *planes) {
    delete planes;
}

size_t libcamera_framebuffer_planes_size(const libcamera_framebuffer_planes_t *planes) {
    return planes->planes.size();
}

libcamera_framebuffer_plane_t *libcamera_framebuffer_planes_at(libcamera_framebuffer_planes_t *planes, size_t index) {
    return &planes->planes.at(index);
}

}

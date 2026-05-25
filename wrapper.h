#include <libheif/heif.h>
#include <libheif/heif_properties.h>
#include <libheif/heif_regions.h>
#include <libheif/heif_items.h>
#include <libheif/heif_sequences.h>
#include <libheif/heif_tai_timestamps.h>
#include <libheif/heif_text.h>
// TODO: Try to enable this in the future.
//       Right now, enabling WITH_UNCOMPRESSED_CODEC option in build.rs
//       is the reason of linker's errors like
//       "undefined reference to `BrotliEncoderDestroyInstance'"
// #include <libheif/heif_uncompressed.h>

// #include <libheif/heif_plugins.h>

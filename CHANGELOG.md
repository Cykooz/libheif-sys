# Change Log

## [5.2.0] - 2026-02-18

## Changed

- Updated version of `vcpkg`. Enabled optional codecs (x264, h264-decoder, jpeg)
  that will be installed from `vcpkg`.

## Fixed

- Use `vcpkg` only for Windows-MSVC ([#12](https://github.com/Cykooz/libheif-sys/pull/12)).

## [5.1.1] - 2026-01-17

### Changes

- Updated embedded source code of `libheif` to version 1.21.2.

## [5.1.0] - 2026-01-16

### Added

- Added feature `v1_21`.
- Updated embedded source code of `libheif` to version 1.21.1.
- Added support of `libheif 1.21`:
    - added new values into `heif_suberror_code` "enum':
        - `heif_suberror_code_heif_suberror_Unsupported_track_type`
    - added new values into `heif_item_property_type` "enum':
        - `heif_item_property_type_heif_item_property_type_extended_language`
    - added new values into `heif_track_type_4cc` "enum':
        - `heif_track_type_4cc_heif_track_type_auxiliary`
    - added enums:
        - `heif_auxiliary_track_info_type`
        - `heif_sequence_gop_structure`
    - struct `heif_security_limits` **was updated to version 3**,
      added new fields:
        - `max_sequence_frames`
        - `max_number_of_file_brands`
    - struct `heif_decoding_options` **was updated to version 8**,
      added new fields:
        - `ignore_sequence_editlist`
        - `output_image_nclx_profile`
        - `num_library_threads`
        - `num_codec_threads`
    - struct `heif_sequence_encoding_options` **was updated to version 2**,
      added new fields:
        - `gop_structure`
        - `keyframe_distance_min`
        - `keyframe_distance_max`
        - `save_alpha_channel`
    - added functions:
        - `heif_context_set_number_of_sequence_repetitions`
        - `heif_image_handle_get_gimi_content_id`
        - `heif_image_handle_set_content_light_level`
        - `heif_image_handle_set_mastering_display_colour_volume`
        - `heif_image_handle_set_pixel_aspect_ratio`
        - `heif_item_get_property_extended_language`
        - `heif_item_set_property_extended_language`
        - `heif_metadata_compression_method_supported`
        - `heif_track_encode_end_of_sequence`
        - `heif_track_get_auxiliary_info_data`
        - `heif_track_get_auxiliary_info_type_urn`
        - `heif_track_get_auxiliary_info_type`
        - `heif_track_has_alpha_channel`

## [5.0.0] - 2025-08-07

### Added

- Added feature `v1_20`.
- Updated embedded source code of `libheif` to version 1.20.2.
- Added support of `libheif 1.20`:
    - added new values into `heif_error_code` "enum':
        - `heif_error_code_heif_error_End_of_sequence`
    - added new values into `heif_suberror_code` "enum':
        - `heif_suberror_code_heif_suberror_No_moov_box`
    - struct `heif_security_limits` **was updated to version 2**, added new fields:
        - `max_total_memory`
        - `max_sample_description_box_entries`
        - `max_sample_group_description_box_entries`
    - struct `heif_decoding_options` **was updated to version 7**, added new fields:
        - `color_conversion_options_ext`
    - added enums:
        - `heif_alpha_composition_mode`
        - `heif_track_type`
        - `heif_sample_aux_info_presence`
        - `heif_track_reference_type`
    - added type aliases:
        - `heif_track_type_4cc`
    - added structs:
        - `heif_color_conversion_options_ext`
        - `heif_track`
        - `heif_raw_sequence_sample`
        - `heif_track_options`
        - `heif_sequence_encoding_options`
        - `heif_sample_aux_info_type`
        - `heif_tai_clock_info`
        - `heif_tai_timestamp_packet`
    - added functions:
        - `heif_string_release`
        - `heif_image_extract_area`
        - `heif_image_get_plane_readonly2`
        - `heif_image_get_plane2`
        - `heif_image_add_plane_safe`
        - `heif_color_conversion_options_ext_alloc`
        - `heif_color_conversion_options_ext_copy`
        - `heif_color_conversion_options_ext_free`
        - `heif_encoding_options_copy`
        - `heif_context_set_major_brand`
        - `heif_decoding_options_copy`
        - `heif_context_has_sequence`
        - `heif_context_get_sequence_timescale`
        - `heif_context_get_sequence_duration`
        - `heif_track_release`
        - `heif_context_number_of_sequence_tracks`
        - `heif_context_get_track_ids`
        - `heif_track_get_id`
        - `heif_context_get_track`
        - `heif_track_get_track_handler_type`
        - `heif_track_get_timescale`
        - `heif_track_get_image_resolution`
        - `heif_track_decode_next_image`
        - `heif_image_get_duration`
        - `heif_track_get_sample_entry_type_of_first_cluster`
        - `heif_track_get_urim_sample_entry_uri_of_first_cluster`
        - `heif_track_get_next_raw_sequence_sample`
        - `heif_raw_sequence_sample_release`
        - `heif_raw_sequence_sample_get_data`
        - `heif_raw_sequence_sample_get_data_size`
        - `heif_raw_sequence_sample_get_duration`
        - `heif_context_set_sequence_timescale`
        - `heif_track_options_alloc`
        - `heif_track_options_release`
        - `heif_track_options_set_timescale`
        - `heif_track_options_set_interleaved_sample_aux_infos`
        - `heif_track_options_enable_sample_tai_timestamps`
        - `heif_track_options_enable_sample_gimi_content_ids`
        - `heif_track_options_set_gimi_track_id`
        - `heif_sequence_encoding_options_alloc`
        - `heif_sequence_encoding_options_release`
        - `heif_context_add_visual_sequence_track`
        - `heif_image_set_duration`
        - `heif_track_encode_sequence_image`
        - `heif_context_add_uri_metadata_sequence_track`
        - `heif_raw_sequence_sample_alloc`
        - `heif_raw_sequence_sample_set_data`
        - `heif_raw_sequence_sample_set_duration`
        - `heif_track_add_raw_sequence_sample`
        - `heif_track_get_number_of_sample_aux_infos`
        - `heif_track_get_sample_aux_info_types`
        - `heif_track_get_gimi_track_content_id`
        - `heif_image_get_gimi_sample_content_id`
        - `heif_raw_sequence_sample_get_gimi_sample_content_id`
        - `heif_image_set_gimi_sample_content_id`
        - `heif_raw_sequence_sample_set_gimi_sample_content_id`
        - `heif_raw_sequence_sample_has_tai_timestamp`
        - `heif_raw_sequence_sample_get_tai_timestamp`
        - `heif_raw_sequence_sample_set_tai_timestamp`
        - `heif_track_get_tai_clock_info_of_first_cluster`
        - `heif_track_add_reference_to_track`
        - `heif_track_get_number_of_track_reference_types`
        - `heif_track_get_track_reference_types`
        - `heif_track_get_number_of_track_reference_of_type`
        - `heif_track_get_references_from_track`
        - `heif_track_find_referring_tracks`
        - `heif_tai_clock_info_alloc`
        - `heif_tai_clock_info_copy`
        - `heif_tai_clock_info_release`
        - `heif_tai_timestamp_packet_alloc`
        - `heif_tai_timestamp_packet_copy`
        - `heif_tai_timestamp_packet_release`
        - `heif_item_set_property_tai_clock_info`
        - `heif_item_get_property_tai_clock_info`
        - `heif_item_set_property_tai_timestamp`
        - `heif_item_get_property_tai_timestamp`
        - `heif_image_set_tai_timestamp`
        - `heif_image_get_tai_timestamp`

### Fixed

- **BREAKING**: Deleted deriving `Copy` and `Clone` for structs with pointers
  and for structs representing `shared_ptr` from C++:
    - `heif_depth_representation_info`
    - `heif_camera_extrinsic_matrix`
    - `heif_context`
    - `heif_image_handle`
    - `heif_decoder_plugin`
    - `heif_encoder_plugin`
    - `heif_image`
    - `heif_scaling_options`
    - `heif_encoder`
    - `heif_reading_options`
    - `heif_encoder_descriptor`
    - `heif_encoder_parameter`
    - `heif_decoder_descriptor`
    - `heif_region_item`
    - `heif_region`

## [4.1.0] - 2025-06-10

### Added

- Added API from `heif_items.h` file from `libheif 1.19`
  ([#11](https://github.com/Cykooz/libheif-sys/pull/11)).

### Changes

- Updated embedded source code of `libheif` to version 1.19.8.

## [4.0.1] - 2025-04-08

### Fixes

- Fixed building of the embedded version of `libheif`.

## [4.0.0] - 2025-04-07

### Added

- **BREAKING**: Feature `use-bindgen` was removed from default features.
- **BREAKING**: Features `compile-libheif` and `embedded-libheif-plugins` were removed.
- The crate `system-deps` is now used instead of `pkg-config` to find
  and link a required version of `libheif`.
- Added features `v1_17`, `v1_18` and `v1_19` to choose a minimal
  version of supported `libheif` API.
- Added embedded source code of `libheif 1.19.7`.
- Added feature `embedded-libheif` to compile and then link statically
  the embedded version of `libheif`.
- Added support of `libheif 1.19`:
    - added new values into `heif_error_code` "enum':
        - `heif_error_code_heif_error_Canceled`
    - added new values into `heif_suberror_code` "enum':
        - `heif_suberror_code_heif_suberror_No_avcC_box`
        - `heif_suberror_code_heif_suberror_Invalid_mini_box`
        - `heif_suberror_code_heif_suberror_Unsupported_essential_property`
    - added new values into `heif_colorspace` "enum':
        - `heif_colorspace_heif_colorspace_nonvisual`
    - added new values into `heif_channel` "enum':
        - `heif_channel_heif_channel_filter_array`
        - `heif_channel_heif_channel_depth`
        - `heif_channel_heif_channel_disparity`
    - added new values into `heif_reader_grow_status` "enum':
        - `heif_reader_grow_status_heif_reader_grow_status_error`
    - added new values into `heif_item_property_type` "enum':
        - `heif_item_property_type_heif_item_property_type_tai_clock_info`
        - `heif_item_property_type_heif_item_property_type_tai_timestamp`
    - struct `heif_reader` **was updated to version 2**, added new fields:
        - `request_range`
        - `preload_range_hint`
        - `release_file_range`
        - `release_error_msg`
    - added new fields into struct `heif_decoding_options`:
        - `cancel_decoding`
    - added type aliases:
        - `heif_entity_group_id`
    - added structs:
        - `heif_reader_range_request_result`
        - `heif_security_limits`
        - `heif_image_tiling`
        - `heif_entity_group`
        - `heif_ambient_viewing_environment`
    - added functions:
        - `heif_read_minor_version_brand`
        - `heif_get_global_security_limits`
        - `heif_get_disabled_security_limits`
        - `heif_context_get_security_limits`
        - `heif_context_set_security_limits`
        - `heif_image_handle_get_image_tiling`
        - `heif_image_handle_get_grid_image_tile_id`
        - `heif_image_handle_decode_image_tile`
        - `heif_context_get_entity_groups`
        - `heif_entity_groups_release`
        - `heif_color_conversion_options_set_defaults`
        - `heif_image_extend_to_size_fill_with_zero`
        - `heif_image_handle_get_content_light_level`
        - `heif_image_handle_get_mastering_display_colour_volume`
        - `heif_image_handle_get_pixel_aspect_ratio`
        - `heif_context_add_grid_image`
        - `heif_context_add_image_tile`
        - `heif_context_add_overlay_image`
        - `heif_item_get_property_uuid_type`

### Changes

- Disabled building of example applications and tests for `libheif`
  if `embedded-libheif` feature is enabled.
- Enabled libheif's cmake options for building all encoders and decoders
  if `embedded-libheif` feature is enabled.

## [3.1.0] - 2024-12-04

### Changes

- `.h` files from `libheif` was embedded in the crate sources.
- Changed `build.rs` to use embedded `.h` files for the bindgen build stage
  instead of `.h` files from the installed libheif library.

  Now you can link the crate with any version of `libheif`
  that is backward compatible with the version supported by the crate.

### Fixes

- Deleted layout tests from `bindings.rs`
  ([#8](https://github.com/Cykooz/libheif-sys/issues/8)).

## [3.0.1] - 2024-11-12

### Added

- Added features to compile `libheif` source-code form GitHub and
  link it statically (not supported for Windows):
    - `compile-libheif`
    - `embedded-libheif-plugins`
- Updated "bindings.rs" to correspond `libheif 1.18.2`:
    - added new values into `heif_suberror_code` "enum':
        - `heif_suberror_code_heif_suberror_No_ispe_property`
        - `heif_suberror_code_heif_suberror_Camera_intrinsic_matrix_undefined`
        - `heif_suberror_code_heif_suberror_Camera_extrinsic_matrix_undefined`
        - `heif_suberror_code_heif_suberror_Invalid_J2K_codestream`
        - `heif_suberror_code_heif_suberror_No_vvcC_box`
        - `heif_suberror_code_heif_suberror_No_icbr_box`
        - `heif_suberror_code_heif_suberror_Decompression_invalid_data`
        - `heif_suberror_code_heif_suberror_Compression_initialisation_error`
        - `heif_suberror_code_heif_suberror_Unsupported_generic_compression_method`
        - `heif_suberror_code_heif_suberror_No_matching_decoder_installed`
    - added new values into `heif_compression_format` "enum':
        - `heif_compression_format_heif_compression_HTJ2K`
    - added new values into `heif_metadata_compression` "enum':
        - `heif_metadata_compression_heif_metadata_compression_zlib`
        - `heif_metadata_compression_heif_metadata_compression_brotli`
    - added field `prefer_uncC_short_form` into struct `heif_encoding_options`
    - added structs:
        - `heif_camera_intrinsic_matrix`
        - `heif_property_user_description`
        - `heif_region_item`
        - `heif_region`
    - added enums:
        - `heif_item_property_type`
        - `heif_transform_mirror_direction`
        - `heif_region_type`
    - added functions:
        - `heif_has_compatible_filetype`
        - `heif_context_add_compatible_brand`
        - `heif_context_encode_grid`
        - `heif_context_add_generic_uri_metadata`
        - `heif_item_get_properties_of_type`
        - `heif_item_get_transformation_properties`
        - `heif_item_get_property_type`
        - `heif_item_get_property_user_description`
        - `heif_item_add_property_user_description`
        - `heif_property_user_description_release`
        - `heif_item_get_property_transform_mirror`
        - `heif_item_get_property_transform_rotation_ccw`
        - `heif_item_get_property_transform_crop_borders`
        - `heif_item_add_raw_property`
        - `heif_item_get_property_raw_size`
        - `heif_item_get_property_raw_data`
        - `heif_image_handle_get_number_of_region_items`
        - `heif_image_handle_get_list_of_region_item_ids`
        - `heif_image_handle_add_region_item`
        - `heif_context_get_region_item`
        - `heif_region_item_get_id`
        - `heif_region_item_release`
        - `heif_region_item_get_reference_size`
        - `heif_region_item_get_number_of_regions`
        - `heif_region_item_get_list_of_regions`
        - `heif_region_release`
        - `heif_region_release_many`
        - `heif_region_get_type`
        - `heif_region_get_point`
        - `heif_region_get_point_transformed`
        - `heif_region_get_rectangle`
        - `heif_region_get_rectangle_transformed`
        - `heif_region_get_ellipse`
        - `heif_region_get_ellipse_transformed`
        - `heif_region_get_polygon_num_points`
        - `heif_region_get_polygon_points`
        - `heif_region_get_polygon_points_transformed`
        - `heif_region_get_polyline_num_points`
        - `heif_region_get_polyline_points`
        - `heif_region_get_polyline_points_transformed`
        - `heif_region_get_referenced_mask_ID`
        - `heif_region_get_inline_mask_data_len`
        - `heif_region_get_inline_mask_data`
        - `heif_region_get_mask_image`
        - `heif_region_item_add_region_point`
        - `heif_region_item_add_region_rectangle`
        - `heif_region_item_add_region_ellipse`
        - `heif_region_item_add_region_polygon`
        - `heif_region_item_add_region_polyline`
        - `heif_region_item_add_region_referenced_mask`
        - `heif_region_item_add_region_inline_mask_data`
        - `heif_region_item_add_region_inline_mask`

### Fixed

- **BREAKING**: Deleted deriving `Copy` and `Clone` for structs with pointers:
    - `heif_plugin_info`
    - `heif_decoding_options`
    - `heif_encoding_options`

## [3.0.0] - 2024-11-07

**YANKED**

## [2.2.1] - 2024-12-02

### Fixes

- Deleted layout tests from `bindings.rs`
  ([#8](https://github.com/Cykooz/libheif-sys/issues/8)).

## [2.2.0] - 2024-11-14

### Changes

- Backported from v3:
    - `.h` files from `libheif` was embedded in the crate sources.
    - Changed `build.rs` to use embedded `.h` files for the bindgen build stage
      instead of `.h` files from the installed libheif library.

      Now you can link the crate with any version of `libheif`
      that is backward compatible with the version supported by the crate.

## [2.1.1] - 2024-05-08

- Fixed minimal required version of `libheif` specified in `build.rs`.

## [2.1.0] - 2023-11-28

- Updated "bindings.rs" to correspond `libheif 1.17.4`:
    - added new values into `heif_compression_format` "enum':
        - `heif_compression_format_heif_compression_mask`
    - added new values into `heif_brand` "enum':
        - `heif_brand_heif_j2ki`
        - `heif_brand_heif_j2is`
    - added functions:
        - `heif_get_plugin_directories`
        - `heif_free_plugin_directories`
        - `heif_image_handle_get_preferred_decoding_colorspace`
        - `heif_image_handle_get_context`
        - `heif_image_handle_get_metadata_item_uri_type`

## [2.0.1] - 2023-11-24

- Changed `build.rs` to fix missing include-directory on macOS
  ([#6](https://github.com/Cykooz/libheif-sys/pull/6)).

## [2.0.0] - 2023-09-12

- Fixed link to crate documentation.
- Updated version of `bindgen` to 0.68.
- Added bindgen settings to copy comments from `heif.h` into generated
  rust file.
- **BREAKING**: Feature `use-binding` added into list of default features.

## [1.16.2] - 2023-09-08

- Fixed minimal required version of `libeif` in `build.rs` script.
- Don't link with `libheif` in case of building documentation for `docs.rs`.

## [1.16.1] - 2023-06-21

- Fixed minimal required version of `libheif` in `README.md`.

## [1.16.0] - 2023-06-21

- Updated version of `bindgen` to 0.66.
- Updated "bindings.rs" to correspond `libheif 1.16.2`:
    - added new values into `heif_suberror_code` "enum":
        - `heif_suberror_code_heif_suberror_Invalid_region_data`
        - `heif_suberror_code_heif_suberror_Invalid_property`
        - `heif_suberror_code_heif_suberror_Item_reference_cycle`
        - `heif_suberror_code_heif_suberror_Encoder_initialization`
        - `heif_suberror_code_heif_suberror_Encoder_encoding`
        - `heif_suberror_code_heif_suberror_Encoder_cleanup`
        - `heif_suberror_code_heif_suberror_Too_many_regions`
    - added new values into `heif_brand` "enum":
        - `heif_brand_heif_vvic`
        - `heif_brand_heif_vvis`
        - `heif_brand_heif_evbi`
        - `heif_brand_heif_evbs`
    - added new values into `heif_compression_format` "enum":
        - `heif_compression_format_heif_compression_VVC`
        - `heif_compression_format_heif_compression_EVC`
        - `heif_compression_format_heif_compression_JPEG2000`
        - `heif_compression_format_heif_compression_uncompressed`
    - added new "enum" `heif_item_property_type` with follow values:
        - `heif_item_property_type_heif_item_property_type_invalid`
        - `heif_item_property_type_heif_item_property_type_user_description`
        - `heif_item_property_type_heif_item_property_type_transform_mirror`
        - `heif_item_property_type_heif_item_property_type_transform_rotation`
        - `heif_item_property_type_heif_item_property_type_transform_crop`
        - `heif_item_property_type_heif_item_property_type_image_size`
    - added new "enum" `heif_transform_mirror_direction` with follow values:
        - `heif_transform_mirror_direction_heif_transform_mirror_direction_vertical`
        - `heif_transform_mirror_direction_heif_transform_mirror_direction_horizontal`
    - added new "enum" `heif_chroma_downsampling_algorithm` with follow values:
        - `heif_chroma_downsampling_algorithm_heif_chroma_downsampling_nearest_neighbor`
        - `heif_chroma_downsampling_algorithm_heif_chroma_downsampling_average`
        - `heif_chroma_downsampling_algorithm_heif_chroma_downsampling_sharp_yuv`
    - added new "enum" `heif_chroma_upsampling_algorithm` with follow values:
        - `heif_chroma_upsampling_algorithm_heif_chroma_upsampling_nearest_neighbor`
        - `heif_chroma_upsampling_algorithm_heif_chroma_upsampling_bilinear`
    - added new "enum" `heif_region_type` with follow values:
        - `heif_region_type_heif_region_type_point`
        - `heif_region_type_heif_region_type_rectangle`
        - `heif_region_type_heif_region_type_ellipse`
        - `heif_region_type_heif_region_type_polygon`
        - `heif_region_type_heif_region_type_referenced_mask`
        - `heif_region_type_heif_region_type_inline_mask`
        - `heif_region_type_heif_region_type_polyline`
    - added structs:
        - `heif_property_user_description`
        - `heif_plugin_info`
        - `heif_color_conversion_options`
        - `heif_content_light_level`
        - `heif_mastering_display_colour_volume`
        - `heif_decoded_mastering_display_colour_volume`
        - `heif_decoder_descriptor`
        - `heif_region_item`
        - `heif_region`
    - added new fields into `heif_decoding_options` struct:
        - `decoder_id`
        - `color_conversion_options`
    - added field `color_conversion_options` into structure `heif_encoding_options`;
    - added functions:
        - `heif_image_handle_get_item_id`
        - `heif_image_handle_release_auxiliary_type`
        - `heif_item_get_properties_of_type`
        - `heif_item_get_transformation_properties`
        - `heif_item_get_property_type`
        - `heif_item_get_property_user_description`
        - `heif_item_add_property_user_description`
        - `heif_property_user_description_release`
        - `heif_item_get_property_transform_mirror`
        - `heif_item_get_property_transform_rotation_ccw`
        - `heif_item_get_property_transform_crop_borders`
        - `heif_image_has_content_light_level`
        - `heif_image_get_content_light_level`
        - `heif_image_set_content_light_level`
        - `heif_image_has_mastering_display_colour_volume`
        - `heif_image_get_mastering_display_colour_volume`
        - `heif_image_set_mastering_display_colour_volume`
        - `heif_mastering_display_colour_volume_decode`
        - `heif_image_get_pixel_aspect_ratio`
        - `heif_image_set_pixel_aspect_ratio`
        - `heif_get_decoder_descriptors`
        - `heif_decoder_descriptor_get_name`
        - `heif_decoder_descriptor_get_id_name`
        - `heif_get_encoder_descriptors`
        - `heif_image_extend_padding_to_size`
        - `heif_image_handle_get_number_of_region_items`
        - `heif_image_handle_get_list_of_region_item_ids`
        - `heif_context_get_region_item`
        - `heif_region_item_get_id`
        - `heif_region_item_release`
        - `heif_region_item_get_reference_size`
        - `heif_region_item_get_number_of_regions`
        - `heif_region_item_get_list_of_regions`
        - `heif_region_release`
        - `heif_region_release_many`
        - `heif_region_get_type`
        - `heif_region_get_point`
        - `heif_region_get_point_transformed`
        - `heif_region_get_rectangle`
        - `heif_region_get_rectangle_transformed`
        - `heif_region_get_ellipse`
        - `heif_region_get_ellipse_transformed`
        - `heif_region_get_polygon_num_points`
        - `heif_region_get_polygon_points`
        - `heif_region_get_polygon_points_transformed`
        - `heif_region_get_polyline_num_points`
        - `heif_region_get_polyline_points`
        - `heif_region_get_polyline_points_transformed`
        - `heif_image_handle_add_region_item`
        - `heif_region_item_add_region_point`
        - `heif_region_item_add_region_rectangle`
        - `heif_region_item_add_region_ellipse`
        - `heif_region_item_add_region_polygon`
        - `heif_region_item_add_region_polyline`

## [1.14.4] - 2023-06-21

- For Windows target [vcpkg crate](https://crates.io/crates/vcpkg) is used
  to find `libheif` installed with help of `vcpkg`.
- Added support of [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
  to install `libheif` with help of `cargo`.

## [1.14.3] - 2023-06-05

- Updated version of `bindgen` to 0.65.
- Use `pkg-config` in `build.rs` ([#1](https://github.com/Cykooz/libheif-sys/pull/1)).

## [1.14.2] - 2023-01-31

- Updated version of `bindgen` to 0.63.0.
- Updated "bindings.rs" to correspond `libheif 1.14.2`:
    - added new values into `heif_error_code` "enum":
      `heif_error_code_heif_error_Plugin_loading_error`.
    - added new values into `heif_suberror_code` "enum":
      `heif_suberror_code_heif_suberror_Unknown_NCLX_color_primaries`,
      `heif_suberror_code_heif_suberror_Unknown_NCLX_transfer_characteristics`,
      `heif_suberror_code_heif_suberror_Unknown_NCLX_matrix_coefficients`,
      `heif_suberror_code_heif_suberror_Unsupported_header_compression_method`,
      `heif_suberror_code_heif_suberror_Plugin_loading_error`,
      `heif_suberror_code_heif_suberror_Plugin_is_not_loaded`,
      `heif_suberror_code_heif_suberror_Cannot_read_plugin_directory`.
    - added new "enum" `heif_plugin_type` with follow values:
      `heif_plugin_type_heif_plugin_type_encoder`,
      `heif_plugin_type_heif_plugin_type_decoder`.
    - added new "enum" `heif_orientation` with follow values:
      `heif_orientation_heif_orientation_normal`,
      `heif_orientation_heif_orientation_flip_horizontally`,
      `heif_orientation_heif_orientation_rotate_180`,
      `heif_orientation_heif_orientation_flip_vertically`,
      `heif_orientation_heif_orientation_rotate_90_cw_then_flip_horizontally`,
      `heif_orientation_heif_orientation_rotate_90_cw`,
      `heif_orientation_heif_orientation_rotate_90_cw_then_flip_vertically`,
      `heif_orientation_heif_orientation_rotate_270_cw`.
    - added new "enum" `heif_metadata_compression` with follow values:
      `heif_metadata_compression_heif_metadata_compression_off`,
      `heif_metadata_compression_heif_metadata_compression_auto`,
      `heif_metadata_compression_heif_metadata_compression_deflate`.
    - added structs: `heif_init_params`, `heif_plugin_info`.
    - added field `strict_decoding` into structure `heif_decoding_options`.
    - added field `image_orientation` into structure `heif_encoding_options`.
    - added functions: `heif_init`, `heif_deinit`, `heif_load_plugin`,
      `heif_load_plugins`, `heif_unload_plugin`, `heif_check_jpeg_filetype`,
      `heif_context_set_max_decoding_threads`,
      `heif_nclx_color_profile_set_color_primaries`,
      `heif_nclx_color_profile_set_transfer_characteristics`,
      `heif_nclx_color_profile_set_matrix_coefficients`,
      `heif_image_get_decoding_warnings`,
      `heif_image_add_decoding_warning`,
      `heif_context_add_XMP_metadata2`.

## [1.12.0] - 2021-05-12

- Updated "bindings.rs" to correspond `libheif 1.12`:
    - added new values into `heif_suberror_code` "enum":
      `heif_suberror_code_heif_suberror_Wrong_tile_image_pixel_depth`.
    - added new functions: `heif_image_handle_is_premultiplied_alpha`,
      `heif_image_set_premultiplied_alpha`,  `heif_image_is_premultiplied_alpha`.

## [1.11.0] - 2021-02-03

- Updated "bindings.rs" to correspond `libheif 1.11`:
    - added new type `heif_brand2`;
    - added new functions: `heif_read_main_brand`,
      `heif_fourcc_to_brand`,  `heif_brand_to_fourcc`,
      `heif_has_compatible_brand`, `heif_list_compatible_brands`,
      `heif_free_list_of_compatible_brands`,
      `heif_image_handle_free_auxiliary_types`,
    - added new fields into `heif_encoding_options` struct:
      `output_nclx_profile`, `macOS_compatibility_workaround_no_nclx_profile`.

## [1.10.0] - 2021-01-14

- Updated "bindings.rs" to correspond `libheif 1.10`:
    - added new values into `heif_error_code` "enum":
      `heif_error_code_heif_error_Color_profile_does_not_exist`.
    - added new functions: `heif_image_handle_get_number_of_auxiliary_images`,
      `heif_image_handle_get_list_of_auxiliary_image_IDs`,
      `heif_image_handle_get_auxiliary_type`,
      `heif_image_handle_get_auxiliary_image_handle`,
      `heif_encoder_parameter_get_valid_integer_values`,
      `heif_encoder_parameter_integer_valid_values`.
    - added new fields into `heif_encoding_options` struct:
      `macOS_compatibility_workaround`,
      `save_two_colr_boxes_when_ICC_and_nclx_available`.

## [1.9.0] - 2020-09-26

- Updated "bindings.rs" to correspond `libheif 1.9`:
    - added new functions: `heif_nclx_color_profile_alloc`,
      `heif_image_get_primary_width`, `heif_image_get_primary_height`,
      `heif_image_crop`.

## [1.8.1] - 2020-08-28

- Fixed `README.md`.

## [1.8.0] - 2020-08-28

- Updated version of `bindgen` to 0.55.1.
- Updated "bindings.rs" to correspond `libheif 1.8`:
    - added new functions: `heif_nclx_color_profile_free`,
      `heif_encoder_descriptor_supports_lossy_compression`,
      `heif_encoder_descriptor_supports_lossless_compression`.
    - added new values into `heif_suberror_code` "enum":
      `heif_suberror_code_heif_suberror_Invalid_pixi_box`,
      `heif_suberror_code_heif_suberror_No_av1C_box`.
    - added new values into `heif_brand` "enum":
      `heif_brand_heif_avif`, `heif_brand_heif_avis`.
    - added new values into `heif_color_primaries` "enum":
      `heif_color_primaries_heif_color_primaries_generic_film`,
      `heif_color_primaries_heif_color_primaries_ITU_R_BT_2020_2_and_2100_0`,
      `heif_color_primaries_heif_color_primaries_SMPTE_ST_428_1`,
      `heif_color_primaries_heif_color_primaries_SMPTE_RP_431_2`,
      `heif_color_primaries_heif_color_primaries_SMPTE_EG_432_1`,
      `heif_color_primaries_heif_color_primaries_EBU_Tech_3213_E`.
    - added new values into `heif_transfer_characteristics` "enum":
      `heif_transfer_characteristics_heif_transfer_characteristic_logarithmic_100`,
      `heif_transfer_characteristics_heif_transfer_characteristic_logarithmic_100_sqrt10`,
      `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2020_2_10bit`,
      `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2020_2_12bit`,
      `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2100_0_PQ`,
      `heif_transfer_characteristics_heif_transfer_characteristic_SMPTE_ST_428_1`,
      `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2100_0_HLG`.
    - added new values into `heif_matrix_coefficients` "enum":
      `heif_matrix_coefficients_heif_matrix_coefficients_ITU_R_BT_2020_2_non_constant_luminance`,
      `heif_matrix_coefficients_heif_matrix_coefficients_ITU_R_BT_2020_2_constant_luminance`,
      `heif_matrix_coefficients_heif_matrix_coefficients_SMPTE_ST_2085`,
      `heif_matrix_coefficients_heif_matrix_coefficients_chromaticity_derived_non_constant_luminance`,
      `heif_matrix_coefficients_heif_matrix_coefficients_chromaticity_derived_constant_luminance`,
      `heif_matrix_coefficients_heif_matrix_coefficients_ICtCp`.
    - added new values into `heif_compression_format` "enum":
      `heif_compression_format_heif_compression_AV1`.
    - added field `convert_hdr_to_8bit` into `heif_decoding_options` struct.

## [1.6.0] - 2019-11-13

- Updated "bindings.rs" to correspond `libheif 1.6`:
    - added new functions: `heif_context_set_maximum_image_size_limit`,
      `heif_context_add_generic_metadata`.

## [1.5.0] - 2019-08-28

- Updated "bindings.rs" to correspond `libheif 1.5`:
    - added new value of `heif_brand` - `heif_brand_heif_msf1 = 10`;
    - added new functions: `heif_get_file_mime_type`, `heif_image_get_color_profile_type`,
      `heif_image_get_raw_color_profile_size`, `heif_image_get_raw_color_profile`,
      `heif_image_get_nclx_color_profile`, `heif_image_get_bits_per_pixel_range`.

## [1.4.2] - 2019-07-16

- Added "libc" as dependency.
- "bindings.rs" has been rebuild, removed not needed definitions.

## [1.4.1] - 2019-05-24

- Added the feature "use-bindgen" to enable generate bindings
  during building time.

## [1.4.0]

- Initial version.

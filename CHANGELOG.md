# Change Log

## [1.10.0] - 2021-01-14

- Updated "bindings.rs" to correspond ``libheif 1.10``:
  - added new values into ``heif_error_code`` "enum":
    ``heif_error_code_heif_error_Color_profile_does_not_exist``.  
  - added new functions: ``heif_image_handle_get_number_of_auxiliary_images``,
    ``heif_image_handle_get_list_of_auxiliary_image_IDs``, 
    ``heif_image_handle_get_auxiliary_type``,
    ``heif_image_handle_get_auxiliary_image_handle``,
    ``heif_encoder_parameter_get_valid_integer_values``,
    ``heif_encoder_parameter_integer_valid_values``.
  - added new fields into ``heif_encoding_options`` struct:
    ``macOS_compatibility_workaround``,
    ``save_two_colr_boxes_when_ICC_and_nclx_available``.

## [1.9.0] - 2020-09-26

- Updated "bindings.rs" to correspond ``libheif 1.9``:
  - added new functions: ``heif_nclx_color_profile_alloc``, 
    ``heif_image_get_primary_width``, ``heif_image_get_primary_height``,
    ``heif_image_crop``.
    
## [1.8.1] - 2020-08-28

- Fixed ``README.md``.

## [1.8.0] - 2020-08-28

- Updated version of ``bindgen`` to 0.55.1.
- Updated "bindings.rs" to correspond ``libheif 1.8``:
  - added new functions: ``heif_nclx_color_profile_free``, 
    ``heif_encoder_descriptor_supports_lossy_compression``,
    ``heif_encoder_descriptor_supports_lossless_compression``.
  - added new values into ``heif_suberror_code`` "enum":
    ``heif_suberror_code_heif_suberror_Invalid_pixi_box``,
    ``heif_suberror_code_heif_suberror_No_av1C_box``.
  - added new values into ``heif_brand`` "enum":
    ``heif_brand_heif_avif``, ``heif_brand_heif_avis``.
  - added new values into ``heif_color_primaries`` "enum":
    ``heif_color_primaries_heif_color_primaries_generic_film``, 
    ``heif_color_primaries_heif_color_primaries_ITU_R_BT_2020_2_and_2100_0``,
    ``heif_color_primaries_heif_color_primaries_SMPTE_ST_428_1``,
    ``heif_color_primaries_heif_color_primaries_SMPTE_RP_431_2``,
    ``heif_color_primaries_heif_color_primaries_SMPTE_EG_432_1``,
    ``heif_color_primaries_heif_color_primaries_EBU_Tech_3213_E``.
  - added new values into ``heif_transfer_characteristics`` "enum":
    ``heif_transfer_characteristics_heif_transfer_characteristic_logarithmic_100``,
    ``heif_transfer_characteristics_heif_transfer_characteristic_logarithmic_100_sqrt10``,
    ``heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2020_2_10bit``,
    ``heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2020_2_12bit``,
    ``heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2100_0_PQ``,
    ``heif_transfer_characteristics_heif_transfer_characteristic_SMPTE_ST_428_1``,
    ``heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2100_0_HLG``.   
  - added new values into ``heif_matrix_coefficients`` "enum":
    ``heif_matrix_coefficients_heif_matrix_coefficients_ITU_R_BT_2020_2_non_constant_luminance``,
    ``heif_matrix_coefficients_heif_matrix_coefficients_ITU_R_BT_2020_2_constant_luminance``,
    ``heif_matrix_coefficients_heif_matrix_coefficients_SMPTE_ST_2085``,
    ``heif_matrix_coefficients_heif_matrix_coefficients_chromaticity_derived_non_constant_luminance``,
    ``heif_matrix_coefficients_heif_matrix_coefficients_chromaticity_derived_constant_luminance``,
    ``heif_matrix_coefficients_heif_matrix_coefficients_ICtCp``.
  - added new values into ``heif_compression_format`` "enum":
    ``heif_compression_format_heif_compression_AV1``.
  - added field ``convert_hdr_to_8bit`` into ``heif_decoding_options`` struct.

## [1.6.0] - 2019-11-13

- Updated "bindings.rs" to correspond ``libheif 1.6``:
  - added new functions: ``heif_context_set_maximum_image_size_limit``, 
    ``heif_context_add_generic_metadata``.

## [1.5.0] - 2019-08-28

- Updated "bindings.rs" to correspond ``libheif 1.5``:
  - added new value of ``heif_brand`` - ``heif_brand_heif_msf1 = 10``;
  - added new functions: ``heif_get_file_mime_type``, ``heif_image_get_color_profile_type``,
    ``heif_image_get_raw_color_profile_size``, ``heif_image_get_raw_color_profile``,
    ``heif_image_get_nclx_color_profile``, ``heif_image_get_bits_per_pixel_range``.

## [1.4.2] - 2019-07-16

- Added "libc" as dependency.
- "bindings.rs" has been rebuild, removed not needed definitions. 

## [1.4.1] - 2019-05-24

- Added the feature "use-bindgen" to enable generate bindings
  during building time.

## [1.4.0]

- Initial version.

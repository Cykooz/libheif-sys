# Change Log

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

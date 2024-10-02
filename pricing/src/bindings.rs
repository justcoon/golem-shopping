// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod golem {
        #[allow(dead_code)]
        pub mod api {
            #[allow(dead_code, clippy::all)]
            pub mod save_snapshot {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_save_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::save();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_save<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                pub trait Guest {
                    /// Saves the component's state into a user-defined snapshot
                    fn save() -> _rt::Vec<u8>;
                }
                #[doc(hidden)]

                macro_rules! __export_golem_api_save_snapshot_0_2_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "golem:api/save-snapshot@0.2.0#save"]
          unsafe extern "C" fn export_save() -> *mut u8 {
            $($path_to_types)*::_export_save_cabi::<$ty>()
          }
          #[export_name = "cabi_post_golem:api/save-snapshot@0.2.0#save"]
          unsafe extern "C" fn _post_return_save(arg0: *mut u8,) {
            $($path_to_types)*::__post_return_save::<$ty>(arg0)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_golem_api_save_snapshot_0_2_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
            }

            #[allow(dead_code, clippy::all)]
            pub mod load_snapshot {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_load_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::load(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_load<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => (),
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                pub trait Guest {
                    /// Tries to load a user-defined snapshot, setting up the worker's state based on it.
                    /// The function can return with a failure to indicate that the update is not possible.
                    fn load(bytes: _rt::Vec<u8>) -> Result<(), _rt::String>;
                }
                #[doc(hidden)]

                macro_rules! __export_golem_api_load_snapshot_0_2_0_cabi{
      ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

        #[export_name = "golem:api/load-snapshot@0.2.0#load"]
        unsafe extern "C" fn export_load(arg0: *mut u8,arg1: usize,) -> *mut u8 {
          $($path_to_types)*::_export_load_cabi::<$ty>(arg0, arg1)
        }
        #[export_name = "cabi_post_golem:api/load-snapshot@0.2.0#load"]
        unsafe extern "C" fn _post_return_load(arg0: *mut u8,) {
          $($path_to_types)*::__post_return_load::<$ty>(arg0)
        }
      };);
    }
                #[doc(hidden)]
                pub(crate) use __export_golem_api_load_snapshot_0_2_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            }
        }
        #[allow(dead_code)]
        pub mod pricing {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct PricingItem {
                    pub price: f32,
                    pub currency: _rt::String,
                    pub zone: _rt::String,
                }
                impl ::core::fmt::Debug for PricingItem {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("PricingItem")
                            .field("price", &self.price)
                            .field("currency", &self.currency)
                            .field("zone", &self.zone)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct Pricing {
                    pub product_id: _rt::String,
                    pub msrp_prices: _rt::Vec<PricingItem>,
                    pub list_prices: _rt::Vec<PricingItem>,
                }
                impl ::core::fmt::Debug for Pricing {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Pricing")
                            .field("product-id", &self.product_id)
                            .field("msrp-prices", &self.msrp_prices)
                            .field("list-prices", &self.list_prices)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_initialize_pricing_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let base7 = arg0;
                    let len7 = arg1;
                    let mut result7 = _rt::Vec::with_capacity(len7);
                    for i in 0..len7 {
                        let base = base7.add(i * 20);
                        let e7 = {
                            let l0 = *base.add(0).cast::<f32>();
                            let l1 = *base.add(4).cast::<*mut u8>();
                            let l2 = *base.add(8).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(12).cast::<*mut u8>();
                            let l5 = *base.add(16).cast::<usize>();
                            let len6 = l5;
                            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);

                            PricingItem {
                                price: l0,
                                currency: _rt::string_lift(bytes3),
                                zone: _rt::string_lift(bytes6),
                            }
                        };
                        result7.push(e7);
                    }
                    _rt::cabi_dealloc(base7, len7 * 20, 4);
                    let base15 = arg2;
                    let len15 = arg3;
                    let mut result15 = _rt::Vec::with_capacity(len15);
                    for i in 0..len15 {
                        let base = base15.add(i * 20);
                        let e15 = {
                            let l8 = *base.add(0).cast::<f32>();
                            let l9 = *base.add(4).cast::<*mut u8>();
                            let l10 = *base.add(8).cast::<usize>();
                            let len11 = l10;
                            let bytes11 = _rt::Vec::from_raw_parts(l9.cast(), len11, len11);
                            let l12 = *base.add(12).cast::<*mut u8>();
                            let l13 = *base.add(16).cast::<usize>();
                            let len14 = l13;
                            let bytes14 = _rt::Vec::from_raw_parts(l12.cast(), len14, len14);

                            PricingItem {
                                price: l8,
                                currency: _rt::string_lift(bytes11),
                                zone: _rt::string_lift(bytes14),
                            }
                        };
                        result15.push(e15);
                    }
                    _rt::cabi_dealloc(base15, len15 * 20, 4);
                    T::initialize_pricing(result7, result15);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_price_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let result2 = T::get_price(_rt::string_lift(bytes0), _rt::string_lift(bytes1));
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Some(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let PricingItem { price: price4, currency: currency4, zone: zone4 } = e;
                            *ptr3.add(4).cast::<f32>() = _rt::as_f32(price4);
                            let vec5 = (currency4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(12).cast::<usize>() = len5;
                            *ptr3.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                            let vec6 = (zone4.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *ptr3.add(20).cast::<usize>() = len6;
                            *ptr3.add(16).cast::<*mut u8>() = ptr6.cast_mut();
                        }
                        None => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_price<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => (),
                        _ => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                            let l3 = *arg0.add(16).cast::<*mut u8>();
                            let l4 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_update_pricing_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let base7 = arg0;
                    let len7 = arg1;
                    let mut result7 = _rt::Vec::with_capacity(len7);
                    for i in 0..len7 {
                        let base = base7.add(i * 20);
                        let e7 = {
                            let l0 = *base.add(0).cast::<f32>();
                            let l1 = *base.add(4).cast::<*mut u8>();
                            let l2 = *base.add(8).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            let l4 = *base.add(12).cast::<*mut u8>();
                            let l5 = *base.add(16).cast::<usize>();
                            let len6 = l5;
                            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);

                            PricingItem {
                                price: l0,
                                currency: _rt::string_lift(bytes3),
                                zone: _rt::string_lift(bytes6),
                            }
                        };
                        result7.push(e7);
                    }
                    _rt::cabi_dealloc(base7, len7 * 20, 4);
                    let base15 = arg2;
                    let len15 = arg3;
                    let mut result15 = _rt::Vec::with_capacity(len15);
                    for i in 0..len15 {
                        let base = base15.add(i * 20);
                        let e15 = {
                            let l8 = *base.add(0).cast::<f32>();
                            let l9 = *base.add(4).cast::<*mut u8>();
                            let l10 = *base.add(8).cast::<usize>();
                            let len11 = l10;
                            let bytes11 = _rt::Vec::from_raw_parts(l9.cast(), len11, len11);
                            let l12 = *base.add(12).cast::<*mut u8>();
                            let l13 = *base.add(16).cast::<usize>();
                            let len14 = l13;
                            let bytes14 = _rt::Vec::from_raw_parts(l12.cast(), len14, len14);

                            PricingItem {
                                price: l8,
                                currency: _rt::string_lift(bytes11),
                                zone: _rt::string_lift(bytes14),
                            }
                        };
                        result15.push(e15);
                    }
                    _rt::cabi_dealloc(base15, len15 * 20, 4);
                    T::update_pricing(result7, result15);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::get();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Pricing {
                                product_id: product_id2,
                                msrp_prices: msrp_prices2,
                                list_prices: list_prices2,
                            } = e;
                            let vec3 = (product_id2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(8).cast::<usize>() = len3;
                            *ptr1.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                            let vec7 = msrp_prices2;
                            let len7 = vec7.len();
                            let layout7 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 20, 4);
                            let result7 = if layout7.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout7);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec7.into_iter().enumerate() {
                                let base = result7.add(i * 20);
                                {
                                    let PricingItem {
                                        price: price4,
                                        currency: currency4,
                                        zone: zone4,
                                    } = e;
                                    *base.add(0).cast::<f32>() = _rt::as_f32(price4);
                                    let vec5 = (currency4.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *base.add(8).cast::<usize>() = len5;
                                    *base.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                                    let vec6 = (zone4.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *base.add(16).cast::<usize>() = len6;
                                    *base.add(12).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                            }
                            *ptr1.add(16).cast::<usize>() = len7;
                            *ptr1.add(12).cast::<*mut u8>() = result7;
                            let vec11 = list_prices2;
                            let len11 = vec11.len();
                            let layout11 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec11.len() * 20, 4);
                            let result11 = if layout11.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout11);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec11.into_iter().enumerate() {
                                let base = result11.add(i * 20);
                                {
                                    let PricingItem {
                                        price: price8,
                                        currency: currency8,
                                        zone: zone8,
                                    } = e;
                                    *base.add(0).cast::<f32>() = _rt::as_f32(price8);
                                    let vec9 = (currency8.into_bytes()).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *base.add(8).cast::<usize>() = len9;
                                    *base.add(4).cast::<*mut u8>() = ptr9.cast_mut();
                                    let vec10 = (zone8.into_bytes()).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *base.add(16).cast::<usize>() = len10;
                                    *base.add(12).cast::<*mut u8>() = ptr10.cast_mut();
                                }
                            }
                            *ptr1.add(24).cast::<usize>() = len11;
                            *ptr1.add(20).cast::<*mut u8>() = result11;
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => (),
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                            let l7 = *arg0.add(12).cast::<*mut u8>();
                            let l8 = *arg0.add(16).cast::<usize>();
                            let base9 = l7;
                            let len9 = l8;
                            for i in 0..len9 {
                                let base = base9.add(i * 20);
                                {
                                    let l3 = *base.add(4).cast::<*mut u8>();
                                    let l4 = *base.add(8).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                    let l5 = *base.add(12).cast::<*mut u8>();
                                    let l6 = *base.add(16).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                            }
                            _rt::cabi_dealloc(base9, len9 * 20, 4);
                            let l14 = *arg0.add(20).cast::<*mut u8>();
                            let l15 = *arg0.add(24).cast::<usize>();
                            let base16 = l14;
                            let len16 = l15;
                            for i in 0..len16 {
                                let base = base16.add(i * 20);
                                {
                                    let l10 = *base.add(4).cast::<*mut u8>();
                                    let l11 = *base.add(8).cast::<usize>();
                                    _rt::cabi_dealloc(l10, l11, 1);
                                    let l12 = *base.add(12).cast::<*mut u8>();
                                    let l13 = *base.add(16).cast::<usize>();
                                    _rt::cabi_dealloc(l12, l13, 1);
                                }
                            }
                            _rt::cabi_dealloc(base16, len16 * 20, 4);
                        }
                    }
                }
                pub trait Guest {
                    fn initialize_pricing(
                        msrp_prices: _rt::Vec<PricingItem>,
                        list_prices: _rt::Vec<PricingItem>,
                    );
                    fn get_price(currency: _rt::String, zone: _rt::String) -> Option<PricingItem>;
                    fn update_pricing(
                        msrp_prices: _rt::Vec<PricingItem>,
                        list_prices: _rt::Vec<PricingItem>,
                    );
                    fn get() -> Option<Pricing>;
                }
                #[doc(hidden)]

                macro_rules! __export_golem_pricing_api_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "golem:pricing/api#initialize-pricing"]
    unsafe extern "C" fn export_initialize_pricing(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,) {
      $($path_to_types)*::_export_initialize_pricing_cabi::<$ty>(arg0, arg1, arg2, arg3)
    }
    #[export_name = "golem:pricing/api#get-price"]
    unsafe extern "C" fn export_get_price(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,) -> *mut u8 {
      $($path_to_types)*::_export_get_price_cabi::<$ty>(arg0, arg1, arg2, arg3)
    }
    #[export_name = "cabi_post_golem:pricing/api#get-price"]
    unsafe extern "C" fn _post_return_get_price(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_price::<$ty>(arg0)
    }
    #[export_name = "golem:pricing/api#update-pricing"]
    unsafe extern "C" fn export_update_pricing(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,) {
      $($path_to_types)*::_export_update_pricing_cabi::<$ty>(arg0, arg1, arg2, arg3)
    }
    #[export_name = "golem:pricing/api#get"]
    unsafe extern "C" fn export_get() -> *mut u8 {
      $($path_to_types)*::_export_get_cabi::<$ty>()
    }
    #[export_name = "cabi_post_golem:pricing/api#get"]
    unsafe extern "C" fn _post_return_get(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get::<$ty>(arg0)
    }
  };);
}
                #[doc(hidden)]
                pub(crate) use __export_golem_pricing_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 28]);
            }
        }
    }
}
mod _rt {

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }

    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }

    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }

    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }

    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_pricing_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::golem::api::save_snapshot::__export_golem_api_save_snapshot_0_2_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::api::save_snapshot);
  $($path_to_types_root)*::exports::golem::api::load_snapshot::__export_golem_api_load_snapshot_0_2_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::api::load_snapshot);
  $($path_to_types_root)*::exports::golem::pricing::api::__export_golem_pricing_api_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::pricing::api);
  )
}
#[doc(inline)]
pub(crate) use __export_pricing_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:pricing:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 531] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x95\x03\x01A\x02\x01\
A\x06\x01B\x03\x01p}\x01@\0\0\0\x04\0\x04save\x01\x01\x04\x01\x1dgolem:api/save-\
snapshot@0.2.0\x05\0\x01B\x04\x01p}\x01j\0\x01s\x01@\x01\x05bytes\0\0\x01\x04\0\x04\
load\x01\x02\x04\x01\x1dgolem:api/load-snapshot@0.2.0\x05\x01\x01B\x0e\x01r\x03\x05\
pricev\x08currencys\x04zones\x04\0\x0cpricing-item\x03\0\0\x01p\x01\x01r\x03\x0a\
product-ids\x0bmsrp-prices\x02\x0blist-prices\x02\x04\0\x07pricing\x03\0\x03\x01\
@\x02\x0bmsrp-prices\x02\x0blist-prices\x02\x01\0\x04\0\x12initialize-pricing\x01\
\x05\x01k\x01\x01@\x02\x08currencys\x04zones\0\x06\x04\0\x09get-price\x01\x07\x04\
\0\x0eupdate-pricing\x01\x05\x01k\x04\x01@\0\0\x08\x04\0\x03get\x01\x09\x04\x01\x11\
golem:pricing/api\x05\x02\x04\x01\x15golem:pricing/pricing\x04\0\x0b\x0d\x01\0\x07\
pricing\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.20\
8.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

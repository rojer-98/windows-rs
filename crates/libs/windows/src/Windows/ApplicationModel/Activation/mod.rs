#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IActivatedEventArgs(::windows::core::IUnknown);
impl IActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgs {}
impl ::core::fmt::Debug for IActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows::core::HRESULT,
    pub PreviousExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT,
    pub SplashScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IActivatedEventArgsWithUser(::windows::core::IUnknown);
impl IActivatedEventArgsWithUser {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivatedEventArgsWithUser> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivatedEventArgsWithUser> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IActivatedEventArgsWithUser> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IActivatedEventArgsWithUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgsWithUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgsWithUser {}
impl ::core::fmt::Debug for IActivatedEventArgsWithUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgsWithUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IApplicationViewActivatedEventArgs(::windows::core::IUnknown);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IApplicationViewActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IApplicationViewActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IApplicationViewActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IApplicationViewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApplicationViewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationViewActivatedEventArgs {}
impl ::core::fmt::Debug for IApplicationViewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationViewActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x930cef4b_b829_40fc_88f4_8513e8a64738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAppointmentsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AddAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderAddAppointmentActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderAddAppointmentActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RoamingId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TimeToShow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TimeToShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToShow: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IBackgroundActivatedEventArgs(::windows::core::IUnknown);
impl IBackgroundActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TaskInstance)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundActivatedEventArgs {}
impl ::core::fmt::Debug for IBackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
impl IBarcodeScannerPreviewActivatedEventArgs {
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBarcodeScannerPreviewActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBarcodeScannerPreviewActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBarcodeScannerPreviewActivatedEventArgs {}
impl ::core::fmt::Debug for IBarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ICachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
impl ICachedFileUpdaterActivatedEventArgs {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CachedFileUpdaterUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICachedFileUpdaterActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICachedFileUpdaterActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICachedFileUpdaterActivatedEventArgs {}
impl ::core::fmt::Debug for ICachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ICameraSettingsActivatedEventArgs(::windows::core::IUnknown);
impl ICameraSettingsActivatedEventArgs {
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceController)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceExtension)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICameraSettingsActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICameraSettingsActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ICameraSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for ICameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ICommandLineActivatedEventArgs(::windows::core::IUnknown);
impl ICommandLineActivatedEventArgs {
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICommandLineActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICommandLineActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ICommandLineActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandLineActivatedEventArgs {}
impl ::core::fmt::Debug for ICommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandLineActivationOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurrentDirectoryPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactActivatedEventArgs(::windows::core::IUnknown);
impl IContactActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactActivatedEventArgs {}
impl ::core::fmt::Debug for IContactActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd627a1c4_c025_4c41_9def_f1eafad075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactCallActivatedEventArgs(::windows::core::IUnknown);
impl IContactCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactCallActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactCallActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactCallActivatedEventArgs {}
impl ::core::fmt::Debug for IContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactMapActivatedEventArgs(::windows::core::IUnknown);
impl IContactMapActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactMapActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactMapActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMapActivatedEventArgs {}
impl ::core::fmt::Debug for IContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Address: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactMessageActivatedEventArgs(::windows::core::IUnknown);
impl IContactMessageActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactMessageActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactMessageActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMessageActivatedEventArgs {}
impl ::core::fmt::Debug for IContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactPanelActivatedEventArgs(::windows::core::IUnknown);
impl IContactPanelActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactPanel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactPanelActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactPanelActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPanelActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPanel: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactPickerActivatedEventArgs(::windows::core::IUnknown);
impl IContactPickerActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactPickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactPickerActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactPickerActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPickerActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactPostActivatedEventArgs(::windows::core::IUnknown);
impl IContactPostActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactPostActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactPostActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPostActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
impl IContactVideoCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactVideoCallActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactVideoCallActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactVideoCallActivatedEventArgs {}
impl ::core::fmt::Debug for IContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactsProviderActivatedEventArgs(::windows::core::IUnknown);
impl IContactsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactsProviderActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContactsProviderActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContactsProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContactsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactsProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IContactsProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactsProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4580dca8_5750_4916_aa52_c0829521eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContinuationActivatedEventArgs(::windows::core::IUnknown);
impl IContinuationActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContinuationActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContinuationActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IContinuationActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IContinuationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContinuationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinuationActivatedEventArgs {}
impl ::core::fmt::Debug for IContinuationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinuationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe58106b5_155f_4a94_a742_c7e08f4e188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IDeviceActivatedEventArgs(::windows::core::IUnknown);
impl IDeviceActivatedEventArgs {
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformationId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDeviceActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDeviceActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IDeviceActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceActivatedEventArgs {}
impl ::core::fmt::Debug for IDeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceInformationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IDevicePairingActivatedEventArgs(::windows::core::IUnknown);
impl IDevicePairingActivatedEventArgs {
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDevicePairingActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDevicePairingActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IDevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingActivatedEventArgs {}
impl ::core::fmt::Debug for IDevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IDialReceiverActivatedEventArgs(::windows::core::IUnknown);
impl IDialReceiverActivatedEventArgs {
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDialReceiverActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDialReceiverActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialReceiverActivatedEventArgs {}
impl ::core::fmt::Debug for IDialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileActivatedEventArgs(::windows::core::IUnknown);
impl IFileActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IFileActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgs {}
impl ::core::fmt::Debug for IFileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(::windows::core::IUnknown);
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileActivatedEventArgsWithCallerPackageFamilyName> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileActivatedEventArgsWithCallerPackageFamilyName> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithCallerPackageFamilyName {}
impl ::core::fmt::Debug for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgsWithCallerPackageFamilyName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(::windows::core::IUnknown);
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringFilesQuery)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileActivatedEventArgsWithNeighboringFiles> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileActivatedEventArgsWithNeighboringFiles> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::InParam<'a, IFileActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithNeighboringFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithNeighboringFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::core::fmt::Debug for IFileActivatedEventArgsWithNeighboringFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgsWithNeighboringFiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFiles_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
impl IFileOpenPickerActivatedEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileOpenPickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileOpenPickerActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileOpenPickerActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IFileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs {}
impl ::core::fmt::Debug for IFileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs2(::windows::core::IUnknown);
impl IFileOpenPickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileOpenPickerActivatedEventArgs2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileOpenPickerActivatedEventArgs2> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs2 {}
impl ::core::fmt::Debug for IFileOpenPickerActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerActivatedEventArgs2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileOpenPickerContinuationEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a IFileOpenPickerContinuationEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a IFileOpenPickerContinuationEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    Files: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs(::windows::core::IUnknown);
impl IFileSavePickerActivatedEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileSavePickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileSavePickerActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileSavePickerActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IFileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs {}
impl ::core::fmt::Debug for IFileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs2(::windows::core::IUnknown);
impl IFileSavePickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileSavePickerActivatedEventArgs2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFileSavePickerActivatedEventArgs2> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs2 {}
impl ::core::fmt::Debug for IFileSavePickerActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerActivatedEventArgs2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileSavePickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileSavePickerContinuationEventArgs {
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a IFileSavePickerContinuationEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a IFileSavePickerContinuationEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    File: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFolderPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFolderPickerContinuationEventArgs {
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a IFolderPickerContinuationEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a IFolderPickerContinuationEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    Folder: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs(::windows::core::IUnknown);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILaunchActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILaunchActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ILaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs {}
impl ::core::fmt::Debug for ILaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs2(::windows::core::IUnknown);
impl ILaunchActivatedEventArgs2 {
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileActivatedInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileActivatedInfo>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILaunchActivatedEventArgs2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILaunchActivatedEventArgs2> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs2 {}
impl ::core::fmt::Debug for ILaunchActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchActivatedEventArgs2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TileActivatedInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILockScreenActivatedEventArgs(::windows::core::IUnknown);
impl ILockScreenActivatedEventArgs {
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Info)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILockScreenActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILockScreenActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ILockScreenActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenActivatedEventArgs {}
impl ::core::fmt::Debug for ILockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILockScreenCallActivatedEventArgs(::windows::core::IUnknown);
impl ILockScreenCallActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILockScreenCallActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILockScreenCallActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenCallActivatedEventArgs {}
impl ::core::fmt::Debug for ILockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPhoneCallActivatedEventArgs(::windows::core::IUnknown);
impl IPhoneCallActivatedEventArgs {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPhoneCallActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPhoneCallActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IPhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhoneCallActivatedEventArgs {}
impl ::core::fmt::Debug for IPhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPickerReturnedActivatedEventArgs(::windows::core::IUnknown);
impl IPickerReturnedActivatedEventArgs {
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickerOperationId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPickerReturnedActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPickerReturnedActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IPickerReturnedActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPickerReturnedActivatedEventArgs {}
impl ::core::fmt::Debug for IPickerReturnedActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPickerReturnedActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PickerOperationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPrelaunchActivatedEventArgs(::windows::core::IUnknown);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrelaunchActivated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrelaunchActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrelaunchActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IPrelaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPrelaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrelaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrelaunchActivatedEventArgs {}
impl ::core::fmt::Debug for IPrelaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrelaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c44717b_19f7_48d6_b046_cf22826eaa74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PrelaunchActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPrint3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
impl IPrint3DWorkflowActivatedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Workflow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrint3DWorkflowActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrint3DWorkflowActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrint3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrint3DWorkflowActivatedEventArgs {}
impl ::core::fmt::Debug for IPrint3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrint3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
impl IPrintTaskSettingsActivatedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskSettingsActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintTaskSettingsActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for IPrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IProtocolActivatedEventArgs(::windows::core::IUnknown);
impl IProtocolActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProtocolActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProtocolActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgs {}
impl ::core::fmt::Debug for IProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(::windows::core::IUnknown);
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IUnknown {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IInspectable {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
impl ::core::fmt::Debug for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
impl IProtocolForResultsActivatedEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProtocolForResultsOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProtocolForResultsActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProtocolForResultsActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolForResultsActivatedEventArgs {}
impl ::core::fmt::Debug for IProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IRestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
impl IRestrictedLaunchActivatedEventArgs {
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SharedContext)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRestrictedLaunchActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRestrictedLaunchActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedLaunchActivatedEventArgs {}
impl ::core::fmt::Debug for IRestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SharedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ISearchActivatedEventArgs(::windows::core::IUnknown);
impl ISearchActivatedEventArgs {
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).QueryText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISearchActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISearchActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ISearchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgs {}
impl ::core::fmt::Debug for ISearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(::windows::core::IUnknown);
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LinguisticDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISearchActivatedEventArgsWithLinguisticDetails> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISearchActivatedEventArgsWithLinguisticDetails> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISearchActivatedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgsWithLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgsWithLinguisticDetails {}
impl ::core::fmt::Debug for ISearchActivatedEventArgsWithLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchActivatedEventArgsWithLinguisticDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc09f33da_08ab_4931_9b7c_451025f21f81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IShareTargetActivatedEventArgs(::windows::core::IUnknown);
impl IShareTargetActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShareOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IShareTargetActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IShareTargetActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareTargetActivatedEventArgs {}
impl ::core::fmt::Debug for IShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplashScreen(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplashScreen {
    type Vtable = ISplashScreen_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ImageLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageLocation: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IStartupTaskActivatedEventArgs(::windows::core::IUnknown);
impl IStartupTaskActivatedEventArgs {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStartupTaskActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStartupTaskActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IStartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStartupTaskActivatedEventArgs {}
impl ::core::fmt::Debug for IStartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileActivatedInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub RecentlyShownNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))]
    RecentlyShownNotifications: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IToastNotificationActivatedEventArgs(::windows::core::IUnknown);
impl IToastNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Argument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserInput)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IToastNotificationActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IToastNotificationActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToastNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for IToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Argument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IUserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl IUserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserDataAccountProviderActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserDataAccountProviderActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDataAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IUserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IViewSwitcherProvider(::windows::core::IUnknown);
impl IViewSwitcherProvider {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::core::IUnknown {
    fn from(value: IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IViewSwitcherProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::core::IUnknown {
    fn from(value: &IViewSwitcherProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::core::IInspectable {
    fn from(value: IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IViewSwitcherProvider> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::core::IInspectable {
    fn from(value: &IViewSwitcherProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IViewSwitcherProvider> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IViewSwitcherProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewSwitcherProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewSwitcherProvider {}
impl ::core::fmt::Debug for IViewSwitcherProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewSwitcherProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f288a6_5c2c_4d27_bac7_7536088f1219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IVoiceCommandActivatedEventArgs(::windows::core::IUnknown);
impl IVoiceCommandActivatedEventArgs {
    #[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVoiceCommandActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVoiceCommandActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IVoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVoiceCommandActivatedEventArgs {}
impl ::core::fmt::Debug for IVoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IWalletActionActivatedEventArgs(::windows::core::IUnknown);
impl IWalletActionActivatedEventArgs {
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActionKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWalletActionActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWalletActionActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IWalletActionActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWalletActionActivatedEventArgs {}
impl ::core::fmt::Debug for IWalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub ActionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))]
    ActionKind: usize,
    pub ActionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl IWebAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAccountProviderActivatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAccountProviderActivatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IWebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IWebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebAuthenticationResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAuthenticationBrokerContinuationEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAuthenticationBrokerContinuationEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::core::fmt::Debug for IWebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AddAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RoamingId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TimeToShow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(::windows::core::IUnknown);
impl BackgroundActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TaskInstance)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundActivatedEventArgs {}
impl ::core::fmt::Debug for BackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBackgroundActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for ::windows::core::InParam<'a, IBackgroundActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerPreviewActivatedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerPreviewActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, IBarcodeScannerPreviewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
impl CachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CachedFileUpdaterUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterActivatedEventArgs {}
impl ::core::fmt::Debug for CachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICachedFileUpdaterActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, ICachedFileUpdaterActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CameraSettingsActivatedEventArgs(::windows::core::IUnknown);
impl CameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceController)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceExtension)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for CameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICameraSettingsActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for ::windows::core::InParam<'a, ICameraSettingsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CommandLineActivatedEventArgs(::windows::core::IUnknown);
impl CommandLineActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivatedEventArgs {}
impl ::core::fmt::Debug for CommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICommandLineActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for ::windows::core::InParam<'a, ICommandLineActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CommandLineActivationOperation(::windows::core::IUnknown);
impl CommandLineActivationOperation {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CurrentDirectoryPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentDirectoryPath)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetExitCode(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExitCode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExitCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandLineActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandLineActivationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivationOperation {}
impl ::core::fmt::Debug for CommandLineActivationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandLineActivationOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
    const IID: ::windows::core::GUID = <ICommandLineActivationOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::core::IUnknown {
    fn from(value: CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::core::IUnknown {
    fn from(value: &CommandLineActivationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for &::windows::core::IUnknown {
    fn from(value: &CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::core::IInspectable {
    fn from(value: CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::core::IInspectable {
    fn from(value: &CommandLineActivationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for &::windows::core::IInspectable {
    fn from(value: &CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CommandLineActivationOperation {}
unsafe impl ::core::marker::Sync for CommandLineActivationOperation {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactCallActivatedEventArgs(::windows::core::IUnknown);
impl ContactCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactCallActivatedEventArgs {}
impl ::core::fmt::Debug for ContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for ::windows::core::InParam<'a, IContactCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactMapActivatedEventArgs(::windows::core::IUnknown);
impl ContactMapActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMapActivatedEventArgs {}
impl ::core::fmt::Debug for ContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactMapActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for ::windows::core::InParam<'a, IContactMapActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactMessageActivatedEventArgs(::windows::core::IUnknown);
impl ContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMessageActivatedEventArgs {}
impl ::core::fmt::Debug for ContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactMessageActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, IContactMessageActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactPanelActivatedEventArgs(::windows::core::IUnknown);
impl ContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactPanel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanelActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactPanelActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, IContactPanelActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactPickerActivatedEventArgs(::windows::core::IUnknown);
impl ContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactPickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPickerActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactPickerActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for ::windows::core::InParam<'a, IContactPickerActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactPostActivatedEventArgs(::windows::core::IUnknown);
impl ContactPostActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPostActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactPostActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for ::windows::core::InParam<'a, IContactPostActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
impl ContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactVideoCallActivatedEventArgs {}
impl ::core::fmt::Debug for ContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactVideoCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, IContactVideoCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct DeviceActivatedEventArgs(::windows::core::IUnknown);
impl DeviceActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformationId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceActivatedEventArgs {}
impl ::core::fmt::Debug for DeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDeviceActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DeviceActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DeviceActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DeviceActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DeviceActivatedEventArgs> for ::windows::core::InParam<'a, IDeviceActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DeviceActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct DevicePairingActivatedEventArgs(::windows::core::IUnknown);
impl DevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingActivatedEventArgs {}
impl ::core::fmt::Debug for DevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDevicePairingActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, IDevicePairingActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct DialReceiverActivatedEventArgs(::windows::core::IUnknown);
impl DialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for DialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialReceiverActivatedEventArgs {}
impl ::core::fmt::Debug for DialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDialReceiverActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IDialReceiverActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct FileActivatedEventArgs(::windows::core::IUnknown);
impl FileActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringFilesQuery)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for FileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileActivatedEventArgs {}
impl ::core::fmt::Debug for FileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IFileActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IFileActivatedEventArgsWithNeighboringFiles> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct FileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
impl FileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileOpenPickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for FileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPickerActivatedEventArgs {}
impl ::core::fmt::Debug for FileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileOpenPickerActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, IFileOpenPickerActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, IFileOpenPickerActivatedEventArgs2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileOpenPickerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IFileOpenPickerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct FileSavePickerActivatedEventArgs(::windows::core::IUnknown);
impl FileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileSavePickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for FileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePickerActivatedEventArgs {}
impl ::core::fmt::Debug for FileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileSavePickerActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, IFileSavePickerActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, IFileSavePickerActivatedEventArgs2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileSavePickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileSavePickerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IFileSavePickerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FolderPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFolderPickerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IFolderPickerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LaunchActivatedEventArgs(::windows::core::IUnknown);
impl LaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileActivatedInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileActivatedInfo>(result__)
        }
    }
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrelaunchActivated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for LaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchActivatedEventArgs {}
impl ::core::fmt::Debug for LaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILaunchActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, IPrelaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LockScreenActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Info)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for ::windows::core::InParam<'a, ILockScreenActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LockScreenCallActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, ILockScreenCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LockScreenComponentActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenComponentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenComponentActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenComponentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenComponentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&LockScreenComponentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct PhoneCallActivatedEventArgs(::windows::core::IUnknown);
impl PhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallActivatedEventArgs {}
impl ::core::fmt::Debug for PhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, IPhoneCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct PickerReturnedActivatedEventArgs(::windows::core::IUnknown);
impl PickerReturnedActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickerOperationId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerReturnedActivatedEventArgs {}
impl ::core::fmt::Debug for PickerReturnedActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerReturnedActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPickerReturnedActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for ::windows::core::InParam<'a, IPickerReturnedActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Workflow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
}
impl ::core::clone::Clone for Print3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowActivatedEventArgs {}
impl ::core::fmt::Debug for Print3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrint3DWorkflowActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for ::windows::core::InParam<'a, IPrint3DWorkflowActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct PrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
impl PrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for PrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTaskSettingsActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IPrintTaskSettingsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ProtocolActivatedEventArgs(::windows::core::IUnknown);
impl ProtocolActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolActivatedEventArgs {}
impl ::core::fmt::Debug for ProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IProtocolActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IProtocolActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
impl ProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProtocolForResultsOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsActivatedEventArgs {}
impl ::core::fmt::Debug for ProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IProtocolForResultsActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IProtocolActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IProtocolForResultsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct RestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
impl RestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SharedContext)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for RestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RestrictedLaunchActivatedEventArgs {}
impl ::core::fmt::Debug for RestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRestrictedLaunchActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, IRestrictedLaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct SearchActivatedEventArgs(::windows::core::IUnknown);
impl SearchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).QueryText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LinguisticDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchActivatedEventArgs {}
impl ::core::fmt::Debug for SearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISearchActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SearchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SearchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SearchActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SearchActivatedEventArgs> for ::windows::core::InParam<'a, ISearchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SearchActivatedEventArgs> for ::windows::core::InParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SearchActivatedEventArgs> for ::windows::core::InParam<'a, IViewSwitcherProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ShareTargetActivatedEventArgs(::windows::core::IUnknown);
impl ShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShareOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for ShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareTargetActivatedEventArgs {}
impl ::core::fmt::Debug for ShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IShareTargetActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, IShareTargetActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct SplashScreen(::windows::core::IUnknown);
impl SplashScreen {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageLocation(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImageLocation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Dismissed<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Dismissed)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDismissed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::clone::Clone for SplashScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplashScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplashScreen {}
impl ::core::fmt::Debug for SplashScreen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplashScreen").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SplashScreen {
    type Vtable = ISplashScreen_Vtbl;
    const IID: ::windows::core::GUID = <ISplashScreen as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
impl ::core::convert::From<SplashScreen> for ::windows::core::IUnknown {
    fn from(value: SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::core::IUnknown {
    fn from(value: &SplashScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SplashScreen> for &::windows::core::IUnknown {
    fn from(value: &SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SplashScreen> for ::windows::core::IInspectable {
    fn from(value: SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::core::IInspectable {
    fn from(value: &SplashScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SplashScreen> for &::windows::core::IInspectable {
    fn from(value: &SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct StartupTaskActivatedEventArgs(::windows::core::IUnknown);
impl StartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartupTaskActivatedEventArgs {}
impl ::core::fmt::Debug for StartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IStartupTaskActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, IStartupTaskActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct TileActivatedInfo(::windows::core::IUnknown);
impl TileActivatedInfo {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Notifications\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub fn RecentlyShownNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RecentlyShownNotifications)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>(result__)
        }
    }
}
impl ::core::clone::Clone for TileActivatedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileActivatedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileActivatedInfo {}
impl ::core::fmt::Debug for TileActivatedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileActivatedInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
    const IID: ::windows::core::GUID = <ITileActivatedInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::core::IUnknown {
    fn from(value: TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::core::IUnknown {
    fn from(value: &TileActivatedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&TileActivatedInfo> for &::windows::core::IUnknown {
    fn from(value: &TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::core::IInspectable {
    fn from(value: TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::core::IInspectable {
    fn from(value: &TileActivatedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&TileActivatedInfo> for &::windows::core::IInspectable {
    fn from(value: &TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ToastNotificationActivatedEventArgs(::windows::core::IUnknown);
impl ToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Argument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserInput)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for ToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IToastNotificationActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, IToastNotificationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl UserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for UserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountProviderActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IUserDataAccountProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct VoiceCommandActivatedEventArgs(::windows::core::IUnknown);
impl VoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandActivatedEventArgs {}
impl ::core::fmt::Debug for VoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IVoiceCommandActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, IVoiceCommandActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct WalletActionActivatedEventArgs(::windows::core::IUnknown);
impl WalletActionActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActionKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletActionActivatedEventArgs {}
impl ::core::fmt::Debug for WalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWalletActionActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for ::windows::core::InParam<'a, IWalletActionActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl WebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for WebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IWebAccountProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct WebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
impl WebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebAuthenticationResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationBrokerContinuationEventArgs {}
impl ::core::fmt::Debug for WebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebAuthenticationBrokerContinuationEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, IWebAuthenticationBrokerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
}
impl ::core::marker::Copy for ActivationKind {}
impl ::core::clone::Clone for ActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationExecutionState {}
impl ::core::clone::Clone for ApplicationExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationExecutionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationExecutionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

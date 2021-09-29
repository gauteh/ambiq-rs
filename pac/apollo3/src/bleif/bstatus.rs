#[doc = "Register `BSTATUS` reader"]
pub struct R(crate::R<BSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSTATUS` writer"]
pub struct W(crate::W<BSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEHREQ` reader - Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
pub struct BLEHREQ_R(crate::FieldReader<bool, bool>);
impl BLEHREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEHREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEHREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEHREQ` writer - Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
pub struct BLEHREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEHREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `BLEHACK` reader - Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
pub struct BLEHACK_R(crate::FieldReader<bool, bool>);
impl BLEHACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEHACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEHACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEHACK` writer - Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
pub struct BLEHACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEHACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Current status of the power state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRST_A {
    #[doc = "0: Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    OFF = 0,
    #[doc = "1: Initialization state. BLEH not powered value."]
    INIT = 1,
    #[doc = "2: Waiting for the powerup of the BLEH value."]
    PWRON = 2,
    #[doc = "3: The BLE Core is powered and active value."]
    ACTIVE = 3,
    #[doc = "6: The BLE Core has entered sleep mode and the power request is inactive value."]
    SLEEP = 6,
    #[doc = "4: The BLE Core is in shutdown mode value."]
    SHUTDOWN = 4,
}
impl From<PWRST_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRST` reader - Current status of the power state machine"]
pub struct PWRST_R(crate::FieldReader<u8, PWRST_A>);
impl PWRST_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRST_A> {
        match self.bits {
            0 => Some(PWRST_A::OFF),
            1 => Some(PWRST_A::INIT),
            2 => Some(PWRST_A::PWRON),
            3 => Some(PWRST_A::ACTIVE),
            6 => Some(PWRST_A::SLEEP),
            4 => Some(PWRST_A::SHUTDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == PWRST_A::OFF
    }
    #[doc = "Checks if the value of the field is `INIT`"]
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        **self == PWRST_A::INIT
    }
    #[doc = "Checks if the value of the field is `PWRON`"]
    #[inline(always)]
    pub fn is_pwron(&self) -> bool {
        **self == PWRST_A::PWRON
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == PWRST_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        **self == PWRST_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == PWRST_A::SHUTDOWN
    }
}
impl core::ops::Deref for PWRST_R {
    type Target = crate::FieldReader<u8, PWRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRST` writer - Current status of the power state machine"]
pub struct PWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRST_A::OFF)
    }
    #[doc = "Initialization state. BLEH not powered value."]
    #[inline(always)]
    pub fn init(self) -> &'a mut W {
        self.variant(PWRST_A::INIT)
    }
    #[doc = "Waiting for the powerup of the BLEH value."]
    #[inline(always)]
    pub fn pwron(self) -> &'a mut W {
        self.variant(PWRST_A::PWRON)
    }
    #[doc = "The BLE Core is powered and active value."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(PWRST_A::ACTIVE)
    }
    #[doc = "The BLE Core has entered sleep mode and the power request is inactive value."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(PWRST_A::SLEEP)
    }
    #[doc = "The BLE Core is in shutdown mode value."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(PWRST_A::SHUTDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `BLEIRQ` reader - Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
pub struct BLEIRQ_R(crate::FieldReader<bool, bool>);
impl BLEIRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEIRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEIRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEIRQ` writer - Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
pub struct BLEIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEIRQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `WAKEUP` reader - Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` writer - Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DCDCFLAG` reader - Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
pub struct DCDCFLAG_R(crate::FieldReader<bool, bool>);
impl DCDCFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDCFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDCFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDCFLAG` writer - Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
pub struct DCDCFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCFLAG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DCDCREQ` reader - Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
pub struct DCDCREQ_R(crate::FieldReader<bool, bool>);
impl DCDCREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDCREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDCREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDCREQ` writer - Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
pub struct DCDCREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SPISTATUS` reader - Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
pub struct SPISTATUS_R(crate::FieldReader<bool, bool>);
impl SPISTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPISTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPISTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPISTATUS` writer - Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
pub struct SPISTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPISTATUS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "State of the BLE Core logic.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum B2MSTATE_A {
    #[doc = "0: Reset State value."]
    RESET = 0,
    #[doc = "1: Sleep state. value."]
    SLEEP = 1,
    #[doc = "2: Standby State value."]
    STANDBY = 2,
    #[doc = "3: Idle state value."]
    IDLE = 3,
    #[doc = "4: Active state. value."]
    ACTIVE = 4,
}
impl From<B2MSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: B2MSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `B2MSTATE` reader - State of the BLE Core logic."]
pub struct B2MSTATE_R(crate::FieldReader<u8, B2MSTATE_A>);
impl B2MSTATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B2MSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<B2MSTATE_A> {
        match self.bits {
            0 => Some(B2MSTATE_A::RESET),
            1 => Some(B2MSTATE_A::SLEEP),
            2 => Some(B2MSTATE_A::STANDBY),
            3 => Some(B2MSTATE_A::IDLE),
            4 => Some(B2MSTATE_A::ACTIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == B2MSTATE_A::RESET
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        **self == B2MSTATE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        **self == B2MSTATE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == B2MSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == B2MSTATE_A::ACTIVE
    }
}
impl core::ops::Deref for B2MSTATE_R {
    type Target = crate::FieldReader<u8, B2MSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2MSTATE` writer - State of the BLE Core logic."]
pub struct B2MSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> B2MSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B2MSTATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset State value."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(B2MSTATE_A::RESET)
    }
    #[doc = "Sleep state. value."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(B2MSTATE_A::SLEEP)
    }
    #[doc = "Standby State value."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(B2MSTATE_A::STANDBY)
    }
    #[doc = "Idle state value."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(B2MSTATE_A::IDLE)
    }
    #[doc = "Active state. value."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(B2MSTATE_A::ACTIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
    #[inline(always)]
    pub fn blehreq(&self) -> BLEHREQ_R {
        BLEHREQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
    #[inline(always)]
    pub fn blehack(&self) -> BLEHACK_R {
        BLEHACK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Current status of the power state machine"]
    #[inline(always)]
    pub fn pwrst(&self) -> PWRST_R {
        PWRST_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
    #[inline(always)]
    pub fn bleirq(&self) -> BLEIRQ_R {
        BLEIRQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
    #[inline(always)]
    pub fn dcdcflag(&self) -> DCDCFLAG_R {
        DCDCFLAG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
    #[inline(always)]
    pub fn dcdcreq(&self) -> DCDCREQ_R {
        DCDCREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
    #[inline(always)]
    pub fn spistatus(&self) -> SPISTATUS_R {
        SPISTATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - State of the BLE Core logic."]
    #[inline(always)]
    pub fn b2mstate(&self) -> B2MSTATE_R {
        B2MSTATE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
    #[inline(always)]
    pub fn blehreq(&mut self) -> BLEHREQ_W {
        BLEHREQ_W { w: self }
    }
    #[doc = "Bit 11 - Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
    #[inline(always)]
    pub fn blehack(&mut self) -> BLEHACK_W {
        BLEHACK_W { w: self }
    }
    #[doc = "Bits 8:10 - Current status of the power state machine"]
    #[inline(always)]
    pub fn pwrst(&mut self) -> PWRST_W {
        PWRST_W { w: self }
    }
    #[doc = "Bit 7 - Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
    #[inline(always)]
    pub fn bleirq(&mut self) -> BLEIRQ_W {
        BLEIRQ_W { w: self }
    }
    #[doc = "Bit 6 - Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 5 - Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
    #[inline(always)]
    pub fn dcdcflag(&mut self) -> DCDCFLAG_W {
        DCDCFLAG_W { w: self }
    }
    #[doc = "Bit 4 - Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
    #[inline(always)]
    pub fn dcdcreq(&mut self) -> DCDCREQ_W {
        DCDCREQ_W { w: self }
    }
    #[doc = "Bit 3 - Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
    #[inline(always)]
    pub fn spistatus(&mut self) -> SPISTATUS_W {
        SPISTATUS_W { w: self }
    }
    #[doc = "Bits 0:2 - State of the BLE Core logic."]
    #[inline(always)]
    pub fn b2mstate(&mut self) -> B2MSTATE_W {
        B2MSTATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE Core status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bstatus](index.html) module"]
pub struct BSTATUS_SPEC;
impl crate::RegisterSpec for BSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bstatus::R](R) reader structure"]
impl crate::Readable for BSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bstatus::W](W) writer structure"]
impl crate::Writable for BSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSTATUS to value 0"]
impl crate::Resettable for BSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

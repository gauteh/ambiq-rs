#[doc = "Register `INTSET` reader"]
pub struct R(crate::R<INTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSET` writer"]
pub struct W(crate::W<INTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSET_SPEC>;
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
impl From<crate::W<INTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B2MSHUTDN` reader - Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)"]
pub struct B2MSHUTDN_R(crate::FieldReader<bool, bool>);
impl B2MSHUTDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2MSHUTDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2MSHUTDN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2MSHUTDN` writer - Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)"]
pub struct B2MSHUTDN_W<'a> {
    w: &'a mut W,
}
impl<'a> B2MSHUTDN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `B2MACTIVE` reader - Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)"]
pub struct B2MACTIVE_R(crate::FieldReader<bool, bool>);
impl B2MACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2MACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2MACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2MACTIVE` writer - Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)"]
pub struct B2MACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> B2MACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `B2MSLEEP` reader - The B2M_STATE from the BLE Core transitioned into the sleep state"]
pub struct B2MSLEEP_R(crate::FieldReader<bool, bool>);
impl B2MSLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2MSLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2MSLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2MSLEEP` writer - The B2M_STATE from the BLE Core transitioned into the sleep state"]
pub struct B2MSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> B2MSLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CQERR` reader - Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions"]
pub struct CQERR_R(crate::FieldReader<bool, bool>);
impl CQERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQERR` writer - Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions"]
pub struct CQERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CQUPD` reader - Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
pub struct CQUPD_R(crate::FieldReader<bool, bool>);
impl CQUPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQUPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQUPD` writer - Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
pub struct CQUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CQUPD_W<'a> {
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
#[doc = "Field `CQPAUSED` reader - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
pub struct CQPAUSED_R(crate::FieldReader<bool, bool>);
impl CQPAUSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQPAUSED` writer - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
pub struct CQPAUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPAUSED_W<'a> {
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
#[doc = "Field `DERR` reader - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
pub struct DERR_R(crate::FieldReader<bool, bool>);
impl DERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERR` writer - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
pub struct DERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DCMP` reader - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
pub struct DCMP_R(crate::FieldReader<bool, bool>);
impl DCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMP` writer - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
pub struct DCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BLECSSTAT` reader - BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
pub struct BLECSSTAT_R(crate::FieldReader<bool, bool>);
impl BLECSSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLECSSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLECSSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLECSSTAT` writer - BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
pub struct BLECSSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLECSSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BLECIRQ` reader - BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
pub struct BLECIRQ_R(crate::FieldReader<bool, bool>);
impl BLECIRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLECIRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLECIRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLECIRQ` writer - BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
pub struct BLECIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLECIRQ_W<'a> {
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
#[doc = "Field `ICMD` reader - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
pub struct ICMD_R(crate::FieldReader<bool, bool>);
impl ICMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICMD` writer - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
pub struct ICMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ICMD_W<'a> {
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
#[doc = "Field `IACC` reader - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
pub struct IACC_R(crate::FieldReader<bool, bool>);
impl IACC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IACC` writer - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
pub struct IACC_W<'a> {
    w: &'a mut W,
}
impl<'a> IACC_W<'a> {
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
#[doc = "Field `B2MST` reader - B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
pub struct B2MST_R(crate::FieldReader<bool, bool>);
impl B2MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2MST` writer - B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
pub struct B2MST_W<'a> {
    w: &'a mut W,
}
impl<'a> B2MST_W<'a> {
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
#[doc = "Field `FOVFL` reader - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
pub struct FOVFL_R(crate::FieldReader<bool, bool>);
impl FOVFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOVFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOVFL` writer - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
pub struct FOVFL_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVFL_W<'a> {
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
#[doc = "Field `FUNDFL` reader - Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
pub struct FUNDFL_R(crate::FieldReader<bool, bool>);
impl FUNDFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUNDFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNDFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNDFL` writer - Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
pub struct FUNDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNDFL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `THR` reader - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
pub struct THR_R(crate::FieldReader<bool, bool>);
impl THR_R {
    pub(crate) fn new(bits: bool) -> Self {
        THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR` writer - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
pub struct THR_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CMDCMP` reader - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
pub struct CMDCMP_R(crate::FieldReader<bool, bool>);
impl CMDCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDCMP` writer - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
pub struct CMDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)"]
    #[inline(always)]
    pub fn b2mshutdn(&self) -> B2MSHUTDN_R {
        B2MSHUTDN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)"]
    #[inline(always)]
    pub fn b2mactive(&self) -> B2MACTIVE_R {
        B2MACTIVE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The B2M_STATE from the BLE Core transitioned into the sleep state"]
    #[inline(always)]
    pub fn b2msleep(&self) -> B2MSLEEP_R {
        B2MSLEEP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions"]
    #[inline(always)]
    pub fn cqerr(&self) -> CQERR_R {
        CQERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub fn cqupd(&self) -> CQUPD_R {
        CQUPD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CQPAUSED_R {
        CQPAUSED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[inline(always)]
    pub fn blecsstat(&self) -> BLECSSTAT_R {
        BLECSSTAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[inline(always)]
    pub fn blecirq(&self) -> BLECIRQ_R {
        BLECIRQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub fn icmd(&self) -> ICMD_R {
        ICMD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
    #[inline(always)]
    pub fn iacc(&self) -> IACC_R {
        IACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[inline(always)]
    pub fn b2mst(&self) -> B2MST_R {
        B2MST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub fn fovfl(&self) -> FOVFL_R {
        FOVFL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
    #[inline(always)]
    pub fn fundfl(&self) -> FUNDFL_R {
        FUNDFL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub fn cmdcmp(&self) -> CMDCMP_R {
        CMDCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)"]
    #[inline(always)]
    pub fn b2mshutdn(&mut self) -> B2MSHUTDN_W {
        B2MSHUTDN_W { w: self }
    }
    #[doc = "Bit 15 - Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)"]
    #[inline(always)]
    pub fn b2mactive(&mut self) -> B2MACTIVE_W {
        B2MACTIVE_W { w: self }
    }
    #[doc = "Bit 14 - The B2M_STATE from the BLE Core transitioned into the sleep state"]
    #[inline(always)]
    pub fn b2msleep(&mut self) -> B2MSLEEP_W {
        B2MSLEEP_W { w: self }
    }
    #[doc = "Bit 13 - Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions"]
    #[inline(always)]
    pub fn cqerr(&mut self) -> CQERR_W {
        CQERR_W { w: self }
    }
    #[doc = "Bit 12 - Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub fn cqupd(&mut self) -> CQUPD_W {
        CQUPD_W { w: self }
    }
    #[doc = "Bit 11 - Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub fn cqpaused(&mut self) -> CQPAUSED_W {
        CQPAUSED_W { w: self }
    }
    #[doc = "Bit 10 - DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub fn derr(&mut self) -> DERR_W {
        DERR_W { w: self }
    }
    #[doc = "Bit 9 - DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bit 8 - BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[inline(always)]
    pub fn blecsstat(&mut self) -> BLECSSTAT_W {
        BLECSSTAT_W { w: self }
    }
    #[doc = "Bit 7 - BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[inline(always)]
    pub fn blecirq(&mut self) -> BLECIRQ_W {
        BLECIRQ_W { w: self }
    }
    #[doc = "Bit 6 - illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub fn icmd(&mut self) -> ICMD_W {
        ICMD_W { w: self }
    }
    #[doc = "Bit 5 - illegal FIFO access interrupt. Asserted when there is a overflow or underflow event"]
    #[inline(always)]
    pub fn iacc(&mut self) -> IACC_W {
        IACC_W { w: self }
    }
    #[doc = "Bit 4 - B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[inline(always)]
    pub fn b2mst(&mut self) -> B2MST_W {
        B2MST_W { w: self }
    }
    #[doc = "Bit 3 - Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub fn fovfl(&mut self) -> FOVFL_W {
        FOVFL_W { w: self }
    }
    #[doc = "Bit 2 - Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
    #[inline(always)]
    pub fn fundfl(&mut self) -> FUNDFL_W {
        FUNDFL_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
    #[doc = "Bit 0 - Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub fn cmdcmp(&mut self) -> CMDCMP_W {
        CMDCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO Master Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](index.html) module"]
pub struct INTSET_SPEC;
impl crate::RegisterSpec for INTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intset::R](R) reader structure"]
impl crate::Readable for INTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intset::W](W) writer structure"]
impl crate::Writable for INTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for INTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

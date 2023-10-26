#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RXDATA_SPEC>;
#[doc = "Field `DATA` reader - Sample Data"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sample Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Rx Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

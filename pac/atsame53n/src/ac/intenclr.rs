#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `COMP0` reader - Comparator 0 Interrupt Enable"]
pub type COMP0_R = crate::BitReader;
#[doc = "Field `COMP0` writer - Comparator 0 Interrupt Enable"]
pub type COMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP1` reader - Comparator 1 Interrupt Enable"]
pub type COMP1_R = crate::BitReader;
#[doc = "Field `COMP1` writer - Comparator 1 Interrupt Enable"]
pub type COMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIN0` reader - Window 0 Interrupt Enable"]
pub type WIN0_R = crate::BitReader;
#[doc = "Field `WIN0` writer - Window 0 Interrupt Enable"]
pub type WIN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0 Interrupt Enable"]
    #[inline(always)]
    pub fn win0(&self) -> WIN0_R {
        WIN0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<INTENCLR_SPEC, 0> {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<INTENCLR_SPEC, 1> {
        COMP1_W::new(self)
    }
    #[doc = "Bit 4 - Window 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn win0(&mut self) -> WIN0_W<INTENCLR_SPEC, 4> {
        WIN0_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

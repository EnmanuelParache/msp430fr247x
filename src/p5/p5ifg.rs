#[doc = "Register `P5IFG` reader"]
pub struct R(crate::R<P5IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5IFG` writer"]
pub struct W(crate::W<P5IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5IFG_SPEC>;
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
impl From<crate::W<P5IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5IFG_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5ifg](index.html) module"]
pub struct P5IFG_SPEC;
impl crate::RegisterSpec for P5IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p5ifg::R](R) reader structure"]
impl crate::Readable for P5IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5ifg::W](W) writer structure"]
impl crate::Writable for P5IFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5IFG to value 0"]
impl crate::Resettable for P5IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

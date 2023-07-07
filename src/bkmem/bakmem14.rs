#[doc = "Register `BAKMEM14` reader"]
pub struct R(crate::R<BAKMEM14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKMEM14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKMEM14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKMEM14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKMEM14` writer"]
pub struct W(crate::W<BAKMEM14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKMEM14_SPEC>;
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
impl From<crate::W<BAKMEM14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKMEM14_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Memory 14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakmem14](index.html) module"]
pub struct BAKMEM14_SPEC;
impl crate::RegisterSpec for BAKMEM14_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bakmem14::R](R) reader structure"]
impl crate::Readable for BAKMEM14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakmem14::W](W) writer structure"]
impl crate::Writable for BAKMEM14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAKMEM14 to value 0"]
impl crate::Resettable for BAKMEM14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

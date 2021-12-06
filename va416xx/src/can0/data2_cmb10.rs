#[doc = "Register `DATA2_CMB10` reader"]
pub type R = crate::R<Data2Cmb10Spec>;
#[doc = "Register `DATA2_CMB10` writer"]
pub type W = crate::W<Data2Cmb10Spec>;
#[doc = "Field `BYTE6` reader - Data Byte 6"]
pub type Byte6R = crate::FieldReader;
#[doc = "Field `BYTE6` writer - Data Byte 6"]
pub type Byte6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE5` reader - Data Byte 5"]
pub type Byte5R = crate::FieldReader;
#[doc = "Field `BYTE5` writer - Data Byte 5"]
pub type Byte5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> Byte6R {
        Byte6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> Byte5R {
        Byte5R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 6"]
    #[inline(always)]
    #[must_use]
    pub fn byte6(&mut self) -> Byte6W<Data2Cmb10Spec> {
        Byte6W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn byte5(&mut self) -> Byte5W<Data2Cmb10Spec> {
        Byte5W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2_cmb10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2_cmb10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data2Cmb10Spec;
impl crate::RegisterSpec for Data2Cmb10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2_cmb10::R`](R) reader structure"]
impl crate::Readable for Data2Cmb10Spec {}
#[doc = "`write(|w| ..)` method takes [`data2_cmb10::W`](W) writer structure"]
impl crate::Writable for Data2Cmb10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA2_CMB10 to value 0"]
impl crate::Resettable for Data2Cmb10Spec {
    const RESET_VALUE: u32 = 0;
}

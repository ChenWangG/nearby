pub struct BleScanRequest;

pub trait Ble {
    type LanguageEnv<'a>;
    fn start_scan<'a>(&self, language_env: Self::LanguageEnv<'a>, request: BleScanRequest);
}

//pub struct NopBle;
//
//impl Ble for NopBle {
//    type LanguageEnv = ();
//
//    fn start_scan(&self, language: Self::LanguageEnv, request: BleScanRequest) {
//        todo!()
//    }
//}
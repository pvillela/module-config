pub trait MakeAppCfg<ACFG> {
    fn make_app_cfg(&self) -> ACFG;
}

pub trait GetCfg0<'a, ACFG, SCFG>: MakeAppCfg<ACFG> {
    fn get_cfg(&self, app_cfg: &'a ACFG) -> SCFG
    where
        SCFG: 'a;
}

pub trait GetCfg<'a, SCFG> {
    fn get_cfg(&'a self) -> SCFG
    where
        SCFG: 'a;
}

impl<ACFG> MakeAppCfg<ACFG> for fn() -> ACFG {
    fn make_app_cfg(&self) -> ACFG {
        self()
    }
}

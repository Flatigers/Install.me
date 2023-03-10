
#[derive(Copy, Clone, Debug)]
pub enum UIMessage {
    PrePage,
    NextPage,
    Page(PagesRoll),
}

#[derive(Copy, Clone, Debug)]
pub enum PagesRoll {
    HomePage,
    InfoPage,
    LicensePage,
    InstallPage,
    FinishPage,
}
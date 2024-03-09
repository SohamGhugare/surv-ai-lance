use rocket::fs::TempFile;

#[derive(FromForm)]
pub struct UploadForm<'r> {
    pub file: TempFile<'r>,
}

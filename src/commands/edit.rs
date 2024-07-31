use crate::error::Result;
use edit::edit as edit_contents;

pub fn edit(profile_name: String) -> Result<()> {
    // TODO
    let contents = edit_contents("")?;

    dbg!(contents);

    Ok(())
}

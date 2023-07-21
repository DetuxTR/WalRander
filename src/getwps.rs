use std::env::join_paths;
use std::{fs, path};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;


pub fn get_files (rw_f_path : Vec<String>) -> Vec<String>
{
    //Single Directory
    // let paths = fs::read_dir(f_path).unwrap();
    // let mut files : Vec<String> = vec![];
    //
    // for mut path in paths
    // {
    //     if let Ok(entry) = path
    //     {
    //         if let Some(file_path) = entry.path().to_str()
    //         {
    //             files.push(file_path.to_string());
    //         }
    //     }
    // }
    let mut files : Vec<String> = vec![];

    for fp in rw_f_path
    {

        let fpath = fs::read_dir(fp).unwrap();
        for mut path in fpath
        {
            if let Ok(entry) = path
            {
                if let Some (file_path) = entry.path().to_str()
                {
                    files.push(file_path.to_string());
                }
            }
        }
    }


     let supportedformats : Vec<&str> = vec!["jpg","png"];
    let mut supportedfiles : Vec<String> = vec![];
    for file in files
    {

        if let Some(extension) = Path::new(&file).extension()
            {
           if let Some(extension_str) = extension.to_str()
               {
                if supportedformats.contains(&extension_str)
                    {
                    if let Ok(rfp) = fs::canonicalize(file)
                        {
                            if let Some(fp) = rfp.to_str()
                            {
                                println!("{}",fp);
                                supportedfiles.push(fp.to_string());
                            }
                        }
                }
           }
       }
   }
    supportedfiles

}
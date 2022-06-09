use std::collections::HashMap;
use std::io::{BufRead, BufReader, Result, stdin};

use regex::Regex;

struct WarningRegex {
  name: String,
  re: Regex,
}

fn main() -> Result<()> {
  let note_re = Regex::new(r"^note: (.+)$").unwrap();
  let warning_re = Regex::new(r"^.*warning: (.+)$").unwrap();
  let standard_warning_re = Regex::new(r"\[-W(.+)\]").unwrap();
  let build_failed_re = Regex::new(r"\*\* BUILD FAILED \*\*").unwrap();
  let error_re = Regex::new(r"^.*error: (.+)$").unwrap();
  let warnings_re = [
    WarningRegex {
      name: "MYmultiple_groups_file_ref".to_string(),
      re: Regex::new(r"The file reference for .+ is a member of multiple groups \(.+ and .+\); this indicates a malformed project\.  Only the membership in one of the groups will be preserved \(but membership in targets will be unaffected\)\.  If you want a reference to the same file in more than one group, please add another reference to the same path\.").unwrap(),
    },
    WarningRegex {
      name: "MYunsupported_deployment_target_version".to_string(),
      re: Regex::new(r"The iOS deployment target '.+' is set to .+, but the range of supported deployment target versions is .+ to .+\. \(in target '.+' from project '.+'\)$").unwrap(),
    },
    WarningRegex {
      name: "MYframework_renamed".to_string(),
      re: Regex::new(r".+ has been renamed\. Use .+ instead\. \(in target '.+' from project '.+'\)").unwrap(),
    },
    WarningRegex {
      name: "MYframework_deprecated".to_string(),
      re: Regex::new(r".+ is deprecated\. Consider migrating to .+ instead\. \(in target '.+' from project '.+'\)").unwrap(),
    },
    WarningRegex {
      name: "MYcategory_method_conflict".to_string(),
      re: Regex::new(r"method '-.+' in category from /.+\(.+\) conflicts with same method from another category").unwrap(),
    },
    WarningRegex {
      name: "MYunsafe_dylib_app_extension".to_string(),
      re: Regex::new(r"linking against a dylib which is not safe for use in application extensions: /.+").unwrap(),
    },
    WarningRegex {
      name: "MYdylib_ios_version_mismatch".to_string(),
      re: Regex::new(r"dylib \(.+\) was built for newer iOS version \(.+\) than being linked \(.+\)").unwrap(),
    },
    WarningRegex {
      name: "MYfile_arch_mismatch".to_string(),
      re: Regex::new(r"ignoring file .+, building for .+ but attempting to link with file built for .+").unwrap(),
    },
    WarningRegex {
      name: "MYclass_constrained_protocol_deprecated".to_string(),
      re: Regex::new(r"using 'class' keyword to define a class-constrained protocol is deprecated; use 'AnyObject' instead").unwrap(),
    },
    WarningRegex {
      name: "MYmultiple_image_sets".to_string(),
      re: Regex::new(r"The image set name .+ is used by multiple image sets\.").unwrap(),
    },
    WarningRegex {
      name: "MYunassigned_children".to_string(),
      re: Regex::new(r"The launch image set .+ has \d+ unassigned children\.").unwrap(),
    },
    WarningRegex {
      name: "MYunnecessary_check".to_string(),
      re: Regex::new(r"unnecessary check for '.+'; enclosing scope ensures guard will always be true").unwrap(),
    },
  ];

  let mut total_warnings = 0;
  let mut unknown_warnings = 0;
  let mut warning_counts = HashMap::new();
  let mut errors = Vec::new();
  let stdin = stdin();
  let reader = BufReader::new(stdin);
  let mut build_failed = false;
  reader
    .lines()
    .filter_map(|line| line.ok())
    .filter(|line| note_re.is_match(line) || warning_re.is_match(line))
    .for_each(|line| {
      if build_failed_re.is_match(line.as_str()) {
        build_failed = true;
      }
      match note_re.captures(line.as_str()) {
        Some(cap) => {
          println!("\x1b[1m[note]:\x1b[0m {}", &cap[1]);
        },
        None => {},
      }
      match error_re.captures(line.as_str()) {
        Some(cap) => {
          errors.push(cap[1].to_string());
        },
        None => {},
      }    
      match warning_re.captures(line.as_str()) {
        Some(cap) => {
          total_warnings += 1;
          match standard_warning_re.captures(&cap[1]) {
            Some(wcap) => {
              let warning_name = wcap[1].to_string();
              println!("\x1b[1m[warning]: {}\x1b[0m {}", warning_name, &cap[1]);
              let count = warning_counts.entry(warning_name).or_insert(0);
              *count += 1;
            },
            None => {
              let mut match_found = false;
              for wr in &warnings_re {
                if wr.re.is_match(&cap[1]) {
                  let warning_name = wr.name.to_string();
                  println!("\x1b[1m[warning]: {}\x1b[0m {}", warning_name, &cap[1]);
                  let count = warning_counts.entry(warning_name).or_insert(0);
                  *count += 1;
                  match_found = true;
                  break;
                }
              }
              if !match_found {
                unknown_warnings += 1;
                println!("\x1b[1m[warning]: UNKNOWN\x1b[0m {}", &cap[1]);
              }
            },
          }
        },
        None => {},
      }
    });

  println!("\x1b[1m[warnings stats]\x1b[0m");
  for (name, count) in warning_counts.iter() {
    println!("\x1b[1m  {}: \x1b[0m{}", name, count);
  }
  println!("\x1b[1m  Unknown warnings: \x1b[0m{}", unknown_warnings);
  println!("\x1b[1m  Total warnings: \x1b[0m{}", total_warnings);
  
  if build_failed {
    println!("\x1b[1mBuild failed\x1b[0m");
    for error in errors {
      println!("  {}", error);
    }
  }
  Ok(())
}

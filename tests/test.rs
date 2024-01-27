#[cfg(test)]
mod tests {
    use clap::{Arg, Command};

    fn cmd(arg_values: Vec<&'static str>) -> Command {
        Command::new("test")
            .args([
                Arg::new("fname").value_name(arg_values[0]),
                Arg::new("from_type").value_name(arg_values[1]),
                Arg::new("to_type").value_name(arg_values[2])
            ])
    }

    #[test]
    fn should_convert_json_to_yaml() {
        let c = cmd(vec!["application", "JSON", "YAML"]);
        c.try_get_matches_from(vec!["fname", "from_type", "to_type"])
            .unwrap_or_else(|e| e.exit());
    }

    #[test]
    fn should_convert_yaml_to_json() {
        let c = cmd(vec!["application", "YAML", "JSON"]);
        c.try_get_matches_from(vec!["fname", "from_type", "to_type"])
            .unwrap_or_else(|e| e.exit());
    }

    #[test]
    fn should_convert_yaml_to_yaml() {
        let c = cmd(vec!["application", "YAML", "YAML"]);
        c.try_get_matches_from(vec!["fname", "from_type", "to_type"])
            .unwrap_or_else(|e| e.exit());
    }

}
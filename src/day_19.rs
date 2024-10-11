use std::{cmp::Ordering, collections::HashMap, ops::Index, str::FromStr};

use anyhow::{bail, Result};
use regex::Regex;

use crate::re_utils;

#[derive(Debug)]
enum Operator {
    LT,
    GT,
}

impl FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            ">" => Operator::GT,
            "<" => Operator::LT,
            _ => return Err(format!("Invalid operator {s}")),
        })
    }
}
#[derive(Debug)]
struct Step {
    part: String,
    value: usize,
    target: String,
    order: Operator,
}

#[derive(Debug)]
struct Workflow {
    id: String,
    steps: Vec<Step>,
    default_target: String,
}

impl Workflow {
    fn process(&self, part: &Part) -> String {
        //println!("Processing workflow {:?} against part {:?}", self, part);
        for step in &self.steps {
            let value = match step.part.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => todo!(),
            };
            //println!("Comparing {} {:?} {}",value,  step.order, step.value);
            let predicate = match step.order {
                Operator::LT => value < step.value,
                Operator::GT => value > step.value,
            };
            if predicate {
                return step.target.to_string();
            }
        }

        self.default_target.clone()
    }
}

impl FromStr for Workflow {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //println!("Trying to parse {s}");
        let re = Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();
        let mut steps: Vec<Step> = Vec::new();
        for sp in s.split(r",") {
            for (_, [part, operator, amount, target]) in re.captures_iter(sp).map(|c| c.extract()) {
                //println!("found {part} {operator} {amount} {target}");
                steps.push(Step {
                    part: part.to_string(),
                    value: amount.parse().unwrap(),
                    target: target.to_string(),
                    order: operator.parse::<Operator>().unwrap(),
                });
            }
        }
        let id = s[..s.find(r"{").unwrap()].to_string();
        let default_target = s.split(r",").last().unwrap();
        let default_target = default_target[..default_target.len() - 1].to_string();

        Ok(Workflow {
            id,
            steps,
            default_target,
        })
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums = re_utils::parse_line_numbers(s).unwrap();
        let (x, m, a, s) = (nums[0], nums[1], nums[2], nums[3]);
        Ok(Part { x, m, a, s })
    }
}

type WorkflowMap = HashMap<String, Workflow>;

pub fn process(lines: &[String], part: usize) -> Result<usize> {
    let mut map: WorkflowMap = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let workflow = line.parse::<Workflow>().unwrap();
        map.insert(workflow.id.clone(), workflow);
    }
    let mut sum: usize = 0;
    for line in lines {
        if line.starts_with("{") {
            let part = line.parse::<Part>().unwrap();
            sum += process_workflow(&map, &part);
        }
    }
    Ok(sum)
}

fn process_workflow(map: &WorkflowMap, part: &Part) -> usize {
    let mut target = "in".to_string();

    loop {
        //println!("Processing target workflow {target}");
        let workflow = map.get(&target).unwrap();
        target = workflow.process(part);

        match target.as_str() {
            "A" => return part.sum(),
            "R" => return 0,
            _ => continue,
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    const INPUT_1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_simple_input() {
        let lines = utils::string_to_lines(INPUT_1.to_string());
        let result = process(&lines, 1);
        assert_eq!(19114, result.unwrap());
    }
}

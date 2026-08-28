use universal_compiler::{check,run};

#[test]
fn first_program_validates_missing_scorecard(){
    let src=include_str!("../../examples/first.univ");
    let out=run(src).expect("first program should run");
    assert_eq!(out.validations,vec!["Employee must have a scorecard"]);
    assert!(out.stdout.is_empty());
}

#[test]
fn has_condition_detects_present_optional_entity(){
    let src=r#"
ENTITY Scorecard
    rating: Number
END
ENTITY Employee
    scorecard: Scorecard?
END
score = Scorecard(rating: 5)
employee = Employee(scorecard: score)
WHEN employee has a scorecard
    print "yes"
OTHERWISE
    print "no"
END
"#;
    let out=run(src).unwrap();
    assert_eq!(out.stdout,vec!["yes"]);
}

#[test]
fn unknown_property_is_semantic_error(){
    let src=r#"
ENTITY Employee
    name: Text
END
employee = Employee(name: "John")
WHEN employee has a scorecard
    print "bad"
END
"#;
    let errors=check(src).unwrap_err();
    assert!(errors.iter().any(|e|e.code=="U3009"));
}

#[test]
fn function_returns_value(){
    let src=include_str!("../../examples/functions.univ");
    let out=run(src).unwrap();
    assert_eq!(out.stdout,vec!["42"]);
}

use assert_cmd::Command;

#[test]
fn grade_minus3() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("-3");
    cmd.assert().success().stdout("Invalid score\n");
}

#[test]
fn grade0() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("0");
    cmd.assert().success().stdout("Failed with F\n");
}

#[test]
fn grade15() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("15");
    cmd.assert().success().stdout("Failed with F\n");
}

#[test]
fn grade52() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("52");
    cmd.assert().success().stdout("D\n");
}

#[test]
fn grade68() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("68");
    cmd.assert().success().stdout("C\n");
}

#[test]
fn grade74() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("74");
    cmd.assert().success().stdout("B\n");
}

#[test]
fn grade89() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("89");
    cmd.assert().success().stdout("A\n");
}

#[test]
fn grade96() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("96");
    cmd.assert().success().stdout("Excellent with A+\n");
}

#[test]
fn grade110() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("110");
    cmd.assert().success().stdout("Invalid score\n");
}

#[test]
fn pyra10() {
    let mut cmd = Command::cargo_bin("pyra").unwrap();
    cmd.arg("10");
    cmd.assert().success().stdout(
        "*
**
***
****
*****
******
*******
********
*********
**********
*********
********
*******
******
*****
****
***
**
*
",
    );
}

#[test]
fn pyraminus10() {
    let mut cmd = Command::cargo_bin("pyra2").unwrap();
    cmd.arg("-10");
    cmd.assert().success().stdout(
        "         *
        **
       ***
      ****
     *****
    ******
   *******
  ********
 *********
**********
 *********
  ********
   *******
    ******
     *****
      ****
       ***
        **
         *
",
    );
}

#[test]
fn tempc10_100_10() {
    let mut cmd = Command::cargo_bin("tempc").unwrap();
    cmd.arg("10").arg("100").arg("10");
    cmd.assert().success().stdout(
        "Fahr\tCelcius
10\t-12.22
20\t-6.67
30\t-1.11
40\t4.44
50\t10.00
60\t15.56
70\t21.11
80\t26.67
90\t32.22
100\t37.78\n",
    );
}

#[test]
fn tempc200_0_25() {
    let mut cmd = Command::cargo_bin("tempc").unwrap();
    cmd.arg("200").arg("0").arg("25");
    cmd.assert().success().stdout(
    "Fahr\tCelcius
200\t93.33
175\t79.44
150\t65.56
125\t51.67
100\t37.78
75\t23.89
50\t10.00
25\t-3.89
0\t-17.78\n",
    );
}

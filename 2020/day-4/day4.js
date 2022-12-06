const fs = require("fs");

const requiredFields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

function validatePassport(passport) {
  const hasRequired = requiredFields.every((field) =>
    passport.hasOwnProperty(field)
  );
  if (!hasRequired) return false;

  console.log(passport);
}

function myTransform(record) {
  record = record.replace("\n", " ");
  const fields = record.split(" ");
  const result = {};
  fields.forEach((field) => {
    const [k, v] = field.split(":");
    result[k] = v;
  });
  return result;
}

function main() {
  const data = fs.readFileSync("input.txt").toString();

  const records = data.split("\n\n");

  const passports = records.map(myTransform);
  passports.forEach(validatePassport);
}

main();

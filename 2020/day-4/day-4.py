from colorama import Fore, Style
import re


required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]


class Passport():
    def __init__(self, fields):
        self.invalid_fields = []
        self.valid_fields = []

        for field in fields:

            (k, v) = field.split(":")

            if (k not in required_fields) and k != "cid":
                raise Exception(f"Unexpected field {k}")

            self.__setattr__(k, v)
            if not self.validate(k, v):
                self.invalid_fields.append(k)
            else:
                self.valid_fields.append(k)

    def __str__(self):
        result = "{\n"
        for k in required_fields:
            if k in self.valid_fields:
                result += f"\t{k}: {Fore.GREEN}{self.__getattribute__(k)}{Style.RESET_ALL},\n"
            elif k in self.invalid_fields:
                result += f"\t{k}: {Fore.RED}{self.__getattribute__(k)}{Style.RESET_ALL},\n"
            else:
                result += f"\t{k}: {Fore.RED}MISSING{Style.RESET_ALL},\n"
        return result + "} " + f"{self.is_valid()}\n"

    def is_valid(self):
        return len(self.invalid_fields) == 0 and set(required_fields).issubset(set(self.valid_fields))

    def validate(self, k, v):
        # byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if k == "byr":
            return bool(re.match(r'\d{4}$', v) and 1920 <= int(v) <= 2002)
        # iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        elif k == "iyr":
            return bool(re.match(r'\d{4}$', v) and 2010 <= int(v) <= 2020)
        # eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        elif k == "eyr":
            return bool(re.match(r'\d{4}$', v) and 2020 <= int(v) <= 2030)
        # hgt (Height) - a number followed by either cm or in:
            #     If cm, the number must be at least 150 and at most 193.
            #     If in, the number must be at least 59 and at most 76.
        elif k == "hgt":
            z = re.match(r'(\d+)(cm|in)$', v)
            if not z:
                return False

            g = z.groups()

            height = int(g[0])
            units = g[1]
            return (units == "cm" and 150 <= height <= 193) or (units == "in" and 59 <= height <= 76)
        # hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        elif k == "hcl":
            return bool(re.match(r'#[0-9a-f]{6}$', v))
        # ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        elif k == "ecl":
            return bool(re.match(r'(amb|blu|brn|gry|grn|hzl|oth)$', v))
        # pid (Passport ID) - a nine-digit number, including leading zeroes.
        elif k == "pid":
            return bool(re.match(r'\d{9}$', v))
        elif k == "cid":  # cid (Country ID) - ignored, missing or not.
            return True
        else:
            raise Exception(f"unknown key {k}")


def main(s):
    records = s.split("\n\n")

    acc = 0
    for record in records:
        fields = record.replace("\n", " ").split(" ")

        passport = Passport(fields)
        print(passport)

        if (passport.is_valid()):
            acc += 1

    print(len(records))
    print("valid:", acc)


if __name__ == "__main__":
    with open("input.txt") as f:
        s = f.read()
        main(s)

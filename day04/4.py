from typing import Iterator, Optional
from dataclasses import dataclass


@dataclass
class Document:
    byr: Optional[str] = None  # (Birth Year)
    iyr: Optional[str] = None  # (Issue Year)
    eyr: Optional[str] = None  # (Expiration Year)
    hgt: Optional[str] = None  # (Height)
    hcl: Optional[str] = None  # (Hair Color)
    ecl: Optional[str] = None  # (Eye Color)
    pid: Optional[str] = None  # (Passport ID)
    cid: Optional[str] = None  # (Country ID)

    def validate(self) -> bool:
        for k, v in self.__dict__.items():
            if not v and k != "cid":
                return False
        return True

    def validate_pt2(self) -> bool:
        if not self.validate():
            return False
        for k, v in self.__dict__.items():
            validator_func = getattr(self, f"validate_{k}")
            if not validator_func(v):
                print(f"Validation failed: {k}:{v}")
                return False
        return True

    @staticmethod
    def validate_byr(v: str) -> bool:
        num = int(v)
        return num >= 1920 and num <= 2002

    @staticmethod
    def validate_iyr(v: str) -> bool:
        num = int(v)
        return num >= 2010 and num <= 2020

    @staticmethod
    def validate_eyr(v: str) -> bool:
        num = int(v)
        return num >= 2020 and num <= 2030

    @staticmethod
    def validate_hgt(v: str) -> bool:
        if v[-2:] not in {"cm", "in"}:
            return False
        num = int(v[:-2])
        if v.endswith("cm"):
            return num >= 150 and num <= 193
        if v.endswith("in"):
            return num >= 59 and num <= 76
        return False

    @staticmethod
    def validate_hcl(v: str) -> bool:
        # fmt:off
        allowed = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                         "a","b", "c", "d", "e", "f"}
        # fmt:on
        return v[0] == "#" and len(v) == 7 and all(c in allowed for c in v[1:])

    @staticmethod
    def validate_ecl(v: str) -> bool:
        return v in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}

    @staticmethod
    def validate_pid(v: str) -> bool:
        return len(v) == 9

    @staticmethod
    def validate_cid(v: str) -> bool:
        return True


def split_entries(inpt: str) -> Iterator[str]:
    entry = ""
    for row in inpt.splitlines():
        if entry and not row:
            yield entry
            entry = ""
        else:
            entry += f" {row.strip()}"
    if entry:
        # yield final entry
        yield entry


def parse_entry(entry: str) -> Document:
    fields = {}
    for kv_pair in entry.split():
        k, v = kv_pair.strip().split(":")
        fields[k] = v
    return Document(**fields)


example = """
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"""
docs = (parse_entry(e) for e in split_entries(example.strip()))
print("example:", sum(doc.validate() for doc in docs))

with open("4.in") as f:
    docs = (parse_entry(e) for e in split_entries(f.read().strip()))
    print("part1:", sum(doc.validate() for doc in docs))

# tests for part2
assert Document.validate_byr("2002")
assert not Document.validate_byr("2003")
assert Document.validate_hgt("60in")
assert Document.validate_hgt("192cm")
assert not Document.validate_hgt("190in")
assert not Document.validate_hgt("190")
assert Document.validate_hcl("#123abc")
assert not Document.validate_hcl("#123abz")
assert not Document.validate_hcl("123abc")
assert Document.validate_ecl("brn")
assert not Document.validate_ecl("wat")
assert Document.validate_pid("000000001")
assert not Document.validate_pid("0123456789")

invalids = """
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"""
for doc in (parse_entry(e) for e in split_entries(invalids.strip())):
    assert not doc.validate_pt2()

valids = """
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    """
for doc in (parse_entry(e) for e in split_entries(valids.strip())):
    assert doc.validate_pt2()

with open("4.in") as f:
    docs = (parse_entry(e) for e in split_entries(f.read().strip()))
    print("part2:", sum(doc.validate_pt2() for doc in docs))

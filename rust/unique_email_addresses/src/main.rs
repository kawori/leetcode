// Every email consists of a local name and a domain name, separated by the @ sign.
// For example, in alice@leetcode.com, alice is the local name, and leetcode.com is the domain name.
// Besides lowercase letters, these emails may contain '.'s or '+'s.
// If you add periods ('.') between some characters in the local name part of an email address, mail sent there will be forwarded to the same address without dots in the local name.  For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the same email address.  (Note that this rule does not apply for domain names.)
// If you add a plus ('+') in the local name, everything after the first plus sign will be ignored. This allows certain emails to be filtered, for example m.y+name@email.com will be forwarded to my@email.com.  (Again, this rule does not apply for domain names.)
// It is possible to use both of these rules at the same time.
// Given a list of emails, we send one email to each address in the list.  How many different addresses actually receive mails?

// Example 1:
// Input: ["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]
// Output: 2
// Explanation: "testemail@leetcode.com" and "testemail@lee.tcode.com" actually receive mails

// Note:
// 1 <= emails[i].length <= 100
// 1 <= emails.length <= 100
// Each emails[i] contains exactly one '@' character.
// All local and domain names are non-empty.
// Local names do not start with a '+' character.

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for addr in emails {
            let at = addr.find('@').unwrap();
            let mut local: &str = &addr[..at];
            let domain: &str = &addr[at..];
            if let Some(plus) = local.find('+') {
                local = &local[..plus];
            }
            let mut valid_addr = local.replace(".", "");
            valid_addr.push_str(domain);
            // println!("{}", valid_addr);
            set.insert(valid_addr);
        }
        set.len() as i32
    }
}

fn main() {
    let emails = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    let ec = emails.clone();
    println!("{:?} -> {}", ec, Solution::num_unique_emails(emails));
}

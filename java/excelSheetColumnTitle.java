// Given a positive integer,
// return its corresponding column title as appear in an Excel sheet.
// For example:
//     1 -> A
//     2 -> B
//     3 -> C
//     ...
//     26 -> Z
//     27 -> AA
//     28 -> AB
//     ...

// Example 1:
// Input: 1
// Output: "A"

// Example 2:
// Input: 28
// Output: "AB"

// Example 3:
// Input: 701
// Output: "ZY"

class excelSheetColumnTitle {
    public static String convertToTitle(int n) {
        StringBuffer sb = new StringBuffer();
        while (n > 0) {
            char ch = (char) ('A' + (n - 1) % 26);
            sb.append(ch);
            n = (n - 1) / 26;
        }
        String ret = new String(sb.reverse());
        return ret;
    }

    public static void main(String[] args) {
        System.out.println(1 + " -> " + convertToTitle(1));
        System.out.println(28 + " -> " + convertToTitle(28));
        System.out.println(701 + " -> " + convertToTitle(701));
    }
}
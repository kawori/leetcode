// Count the number of segments in a string,
// where a segment is defined to be a contiguous sequence of non-space characters.

// Please note that the string does not contain any non-printable characters.

// Example:
// Input: "Hello, my name is John"
// Output: 5

class numberofSegmentsinaString {
    public static int countSegments(String s) {
        int ret = 0;

        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) != ' ' && (i == 0 || s.charAt(i - 1) == ' ')) {
                ret++;
            }
        }

        return ret;
    }

    public static void main(String[] args) {
        String s = "Hello, my name is John";
        System.out.println("\"" + s + "\" -> " + countSegments(s));
    }
}
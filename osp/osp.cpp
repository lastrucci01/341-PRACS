#include <iostream>
#include <cstring>
#include <iomanip>
#include <cstdlib>
#include <string>
#include <cctype>
using namespace std;
// global
string s = "BABACBABACACACBABACBA";

string pattern_match()
{
    if (s.empty())
    {
        return "empty";
    }

    if (s.substr(0, 4) == "ACBA")
    {
        s = s.substr(4);
        s.append("X");

        string out = pattern_match();
        if (out == "error")
        {
            s.pop_back();
            s = "ACBA" + s;
            if (s.substr(0, 2) == "AC")
            {
                s = s.substr(2);
                s.append("Z");
                pattern_match();
            }
        }
    }
    else if (s.substr(0, 3) == "BAB")
    {
        s = s.substr(3);
        s.append("Y");

        pattern_match();
    }
    else if (s.substr(0, 2) == "AC")
    {
        s = s.substr(2);
        s.append("Z");
        pattern_match();
    }

    if (s[0] == 'X' || s[0] == 'Y' || s[0] == 'Z')
    {
        return "complete";
    }
    else
    {
        return "error";
    }
}

int main()
{
    string out = pattern_match();
    if (out != "error" && out != "empty") 
    {
        cout << s << endl;
    } else cout << out << endl;

    return 0;
}

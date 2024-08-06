#include <iostream>
#include <map>

int check[2001] = { 0, };

class Trie {
public:
    void insert(const char* s)
    {
        if (!*s)
        {
            is_final = true;
            return;
        }

        int c = *s - 'a';

        if (childs.find(c) == childs.end())
        {
            childs[c] = new Trie();
        }

        childs[c]->insert(s + 1);
    }

    void find(const char* s, int idx, bool is_color)
    {
        if (is_final)
        {
            ++check[idx];
        }

        if (!*s)
        {
            return;
        }

        int c = *s - 'a';

        if (childs.find(c) == childs.end())
        {
            return;
        }

        childs[c]->find(s + 1, is_color ? idx + 1 : idx - 1, is_color);
    }

private:
    std::map<char, Trie*> childs;
    bool is_final = false;
};

int main(int argc, char* argv[])
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int c, n;
    std::cin >> c >> n;

    Trie* trie_color = new Trie();
    Trie* trie_nickname = new Trie();

    for (int i = 0; i < c; ++i)
    {
        std::string color;
        std::cin >> color;

        trie_color->insert(color.c_str());
    }

    for (int i = 0; i < n; ++i)
    {
        std::string nickname;
        std::cin >> nickname;

        std::reverse(nickname.begin(), nickname.end());
        trie_nickname->insert(nickname.c_str());
    }

    int q;
    std::cin >> q;

    for (int i = 0; i < q; ++i)
    {
        std::string teamName;
        std::cin >> teamName;

        std::fill(check, check + 2001, 0);

        trie_color->find(teamName.c_str(), 0, true);
        std::reverse(teamName.begin(), teamName.end());
        trie_nickname->find(teamName.c_str(), teamName.size(), false);

        bool ret = false;

        for (int j = 0; j < teamName.size(); ++j)
        {
            if (check[j] == 2)
            {
                ret = true;
                break;
            }
        }

        if (ret)
        {
            std::cout << "Yes\n";
        }
        else
        {
            std::cout << "No\n";
        }
    }
    
	return 0;
}
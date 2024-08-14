#include "url.hpp"

using namespace lib;
using namespace std;

Url::Url(
    string subject
) {
    smatch results;

    static const regex pattern( // @TODO user:password@#fragment?
        R"regex(^(\w+)://([^:\/]+):?(\d+)?\/?([^\?]+)?\??(.*)?$)regex"
    );

    regex_search(
        subject,
        results,
        pattern
    );

    scheme = results[1];
    host   = results[2];
    port   = results[3];
    path   = results[4];
    query  = results[5];
}

string Url::to_string()
{
    string result;

    if (!scheme.empty()) result += scheme + "://";
    if (!host.empty())   result += host;
    if (!port.empty())   result += ":" + port;
    if (!path.empty())   result += "/" + path;
    if (!query.empty())  result += "?" + query;

    return result;
}

Url::~Url() = default;
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

Url::~Url() = default;
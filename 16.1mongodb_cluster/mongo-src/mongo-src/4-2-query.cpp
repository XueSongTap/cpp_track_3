// Copyright 2015 MongoDB Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <iostream>

#include <bsoncxx/builder/basic/array.hpp>
#include <bsoncxx/builder/basic/document.hpp>
#include <bsoncxx/builder/basic/kvp.hpp>
#include <bsoncxx/json.hpp>

#include <mongocxx/client.hpp>
#include <mongocxx/instance.hpp>
#include <mongocxx/options/find.hpp>
#include <mongocxx/uri.hpp>

using bsoncxx::builder::basic::kvp;
using bsoncxx::builder::basic::make_array;
using bsoncxx::builder::basic::make_document;
// g++ --std=c++17 4-2-query.cpp -o 4-2-query $(pkg-config --cflags --libs libmongocxx)

int main(int, char**) {
    // The mongocxx::instance constructor and destructor initialize and shut down the driver,
    // respectively. Therefore, a mongocxx::instance must be created before using the driver and
    // must remain alive for as long as the driver is in use.
    mongocxx::instance inst{};
    mongocxx::client conn{mongocxx::uri{}};

    auto db = conn["test"];
    std::cout << "1-Query for all the documents in a collection." << std::endl;
    // Query for all the documents in a collection.
    {
        // @begin: cpp-query-all
        auto cursor = db["restaurants"].find({});
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;  // bsoncxx::to_json(doc)转成json
        }
        // @end: cpp-query-all
    }
    std::cout << "\n2-Query for equality on a top level field." << std::endl;
    // Query for equality on a top level field.
    {
        // @begin: cpp-query-top-level-field
        auto cursor = db["restaurants"].find(make_document(kvp("borough", "Manhattan")));

        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-top-level-field
    }
    std::cout << "\n3-Query by a field in an embedded document." << std::endl;
    // Query by a field in an embedded document. 嵌入的文档
    {
        // @begin: cpp-query-embedded-document
        auto cursor = db["restaurants"].find(make_document(kvp("address.zipcode", "10075")));
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-embedded-document
    }
    std::cout << "\n4-Query by a field in an array." << std::endl;
    // Query by a field in an array.
    {
        // @begin: cpp-query-field-in-array
        auto cursor = db["restaurants"].find(make_document(kvp("grades.grade", "B")));
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-field-in-array
    }
    std::cout << "\n5-Query with the greater-than operator ($gt)." << std::endl;
    // Query with the greater-than operator ($gt).
    {
        // @begin: cpp-query-greater-than
        auto cursor = db["restaurants"].find(
            make_document(kvp("grade.score", make_document(kvp("$gt", 30)))));
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-greater-than
    }
    std::cout << "\n6-Query with the less-than operator ($lt)." << std::endl;
    // Query with the less-than operator ($lt).
    {
        // @begin: cpp-query-less-than
        auto cursor = db["restaurants"].find(
            make_document(kvp("grades.score", make_document(kvp("$lt", 10)))));
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-less-than
    }
    std::cout << "\n7-Query with a logical conjunction (AND) of query conditions.." << std::endl;
    // Query with a logical conjunction (AND) of query conditions.  与
    {
        // @begin: cpp-query-logical-and
        auto cursor = db["restaurants"].find(
            make_document(kvp("cuisine", "Italian"), kvp("address.zipcode", "10075")));
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-logical-and
    }
    std::cout << "\n8-Query with a logical disjunction (OR) of query conditions." << std::endl;
    // Query with a logical disjunction (OR) of query conditions. 或
    {
        // @begin: cpp-query-logical-or
        auto cursor = db["restaurants"].find(
            make_document(kvp("$or",  // 或
                              make_array(make_document(kvp("cuisine", "Italian")),
                                         make_document(kvp("address.zipcode", "10075"))))));
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-logical-or
    }
    std::cout << "\n9-Sort query results." << std::endl;
    // Sort query results.
    {
        // @begin: cpp-query-sort
        mongocxx::options::find opts;   //  其中 1 为升序排列，而-1是用于降序排列. 字符串注意大小写
        // opts.sort(make_document(kvp("borough", -1), kvp("address.zipcode", 1)));
        opts.sort(make_document(kvp("borough", -1)));

        auto cursor = db["restaurants"].find({}, opts);
        for (auto&& doc : cursor) {
            std::cout << bsoncxx::to_json(doc) << std::endl;
        }
        // @end: cpp-query-sort
    }
}

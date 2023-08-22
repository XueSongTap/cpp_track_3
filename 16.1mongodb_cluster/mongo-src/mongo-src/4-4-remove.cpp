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
#include <bsoncxx/builder/basic/document.hpp>
#include <bsoncxx/builder/basic/kvp.hpp>

#include <mongocxx/client.hpp>
#include <mongocxx/instance.hpp>
#include <mongocxx/uri.hpp>

using bsoncxx::builder::basic::kvp;
using bsoncxx::builder::basic::make_document;
// g++ --std=c++17 4-4-remove.cpp -o 4-4-remove $(pkg-config --cflags --libs libmongocxx)
int main(int, char**) {
    // The mongocxx::instance constructor and destructor initialize and shut down the driver,
    // respectively. Therefore, a mongocxx::instance must be created before using the driver and
    // must remain alive for as long as the driver is in use.
    mongocxx::instance inst{};
    mongocxx::client conn{mongocxx::uri{}};

    auto db = conn["test"];
    std::cout << "\n1-Remove all documents that match a condition." << std::endl;
    // Remove all documents that match a condition.
    {
        // @begin: cpp-remove-matching-documents
        db["restaurants"].delete_many(make_document(kvp("borough", "Manhattan")));
        // @end: cpp-remove-matching-documents
    }

    std::cout << "\n2-Remove one document that matches a condition." << std::endl;
    // Remove one document that matches a condition.
    {
        // @begin: cpp-remove-justone
        db["restaurants"].delete_one(make_document(kvp("borough", "Queens")));
        // @end: cpp-remove-justone
    }

    std::cout << "\n3-Remove all documents in a collection." << std::endl;
    // Remove all documents in a collection.
    {
        // @begin: cpp-remove-all-documents
        db["restaurants"].delete_many({});
        // @end: cpp-remove-all-documents
    }

    std::cout << "\n4-Drop a collection." << std::endl;
    // Drop a collection.
    {
        // @begin: cpp-drop-collection
        db["restaurants"].drop();
        // @end: cpp-drop-collection
    }
}

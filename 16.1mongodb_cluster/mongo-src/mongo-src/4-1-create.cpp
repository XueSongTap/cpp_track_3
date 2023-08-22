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

#include <chrono>

#include <bsoncxx/builder/stream/document.hpp>
#include <bsoncxx/builder/stream/helpers.hpp>
#include <bsoncxx/document/element.hpp>
#include <bsoncxx/stdx/string_view.hpp>
#include <bsoncxx/types.hpp>
#include <bsoncxx/types/bson_value/make_value.hpp>
#include <bsoncxx/types/bson_value/value.hpp>
#include <bsoncxx/types/bson_value/view.hpp>
#include <bsoncxx/json.hpp>     // from json，to json

#include <mongocxx/client.hpp>
#include <mongocxx/instance.hpp>
#include <mongocxx/uri.hpp>

using bsoncxx::builder::basic::kvp;
using bsoncxx::builder::basic::make_array;
using bsoncxx::builder::basic::make_document;
// g++ --std=c++17 4-1-create.cpp -o 4-1-create $(pkg-config --cflags --libs libmongocxx)

int main(int, char**) {
    // The mongocxx::instance constructor and destructor initialize and shut down the driver,
    // respectively. Therefore, a mongocxx::instance must be created before using the driver and
    // must remain alive for as long as the driver is in use.
    mongocxx::instance inst{};
    mongocxx::client conn{mongocxx::uri{}}; // 默认localhost 27017

    auto db = conn["test"];     // 连接对应的数据库获取集合


    // Drop a collection.
    {
        // @begin: cpp-drop-collection
        db["restaurants"].drop();
        // @end: cpp-drop-collection
    }

    // @begin: cpp-insert-a-document
    bsoncxx::document::value restaurant_doc = make_document(
        kvp("address",
            make_document(kvp("street", "2 Avenue"),
                          kvp("zipcode", 10075),
                          kvp("building", "1480"),
                          kvp("coord", make_array(-73.9557413, 40.7720266)))),
        kvp("borough", "Manhattan"),
        kvp("cuisine", "Italian"),
        kvp("grades",
            make_array(
                make_document(kvp("date", bsoncxx::types::b_date{std::chrono::milliseconds{12323}}),
                              kvp("grade", "A"),
                              kvp("score", 11)),
                make_document(
                    kvp("date", bsoncxx::types::b_date{std::chrono::milliseconds{121212}}),
                    kvp("grade", "B"),
                    kvp("score", 17)))),
        kvp("name", "Vella"),
        kvp("restaurant_id", "41704620"));

    // We choose to move in our document here, which transfers ownership to insert_one()
    auto res = db["restaurants"].insert_one(std::move(restaurant_doc));

    bsoncxx::document::value restaurant_doc2 = make_document(
        kvp("address",
            make_document(kvp("street", "3 Avenue"),
                          kvp("zipcode", 10088),
                          kvp("building", "2080"),
                          kvp("coord", make_array(-13.9557413, 70.7720266)))),
        kvp("borough", "Changsha"),    // 自治市镇; (城市)行政区;
        kvp("cuisine", "china"), // 烹饪; 风味; (通常指昂贵的饭店中的)饭菜，菜肴
        kvp("grades", // 等级，品级
            make_array(
                make_document(kvp("date", bsoncxx::types::b_date{std::chrono::milliseconds{12323}}),
                              kvp("grade", "A+"),
                              kvp("score", 20)),
                make_document(
                    kvp("date", bsoncxx::types::b_date{std::chrono::milliseconds{121212}}),
                    kvp("grade", "A"),
                    kvp("score", 17)))),
        kvp("name", "Vella"),
        kvp("restaurant_id", "100000"));

    // We choose to move in our document here, which transfers ownership to insert_one()
    auto res2 = db["restaurants"].insert_one(std::move(restaurant_doc2));
    
    
    // 这个json没有实际的意义，纯粹是为了演示能够把json转成mongodb需要的格式
    std::string  json = "{\"praenomen\":\"Gaius\",\"nomen\":\"Julius\",\"cognomen\":\"Caezar\"," "\"born\":-100,\"died\":-44}";

    bsoncxx::document::value restaurant_doc3 = bsoncxx::from_json(json); // json -> bsoncxx::document
    auto res3 = db["restaurants"].insert_one(std::move(restaurant_doc3));

    // mongo-src\mongo-driver\mongo-cxx-driver\docs\content
    // @end: cpp-insert-a-document
}

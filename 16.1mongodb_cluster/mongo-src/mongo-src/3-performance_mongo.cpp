// performance_mongo.cpp, 实际是根据example的create.cpp修改
#include <chrono>

#include <bsoncxx/builder/basic/array.hpp>
#include <bsoncxx/builder/basic/document.hpp>
#include <bsoncxx/builder/basic/kvp.hpp>
#include <bsoncxx/types.hpp>

#include <mongocxx/client.hpp>
#include <mongocxx/instance.hpp>
#include <mongocxx/uri.hpp>
#include <time.h>
#include <string>
#include <iostream>
using bsoncxx::builder::basic::kvp;
using bsoncxx::builder::basic::make_array;
using bsoncxx::builder::basic::make_document;
// g++ --std=c++11 3-performance_mongo.cpp -o 3-performance_mongo $(pkg-config --cflags --libs libmongocxx)
int main(int, char **)
{

    mongocxx::instance inst{};
    // 连接复制集集群
    // mongocxx::uri url{"mongodb://localhost:27017"};
    mongocxx::uri url{"mongodb://localhost:28017,localhost:28018,localhost:28019/?replicaSet=rs0"}; 
    mongocxx::client conn{url};     

    // 连接单机
     mongocxx::client conn{mongocxx::uri{}};     // 缺省  "mongodb://localhost:27017";

    auto db = conn["test"]; 

    // We choose to move in our document here, which transfers ownership to insert_one()
    clock_t startTime = clock();
    for (int i = 0; i <= 10000; ++i)     // make_document对象，make_array数组
    {
        // 封装一个文档
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
            kvp("restaurant_id", std::to_string(i)));
        // 插入数据库
        auto res = db["restaurants"].insert_one(std::move(restaurant_doc));
    }
    /*
        {
        "_id" : ObjectId("60dc372e25794931d61d1959"),
        "address" : {
            "street" : "2 Avenue",
            "zipcode" : 10075,
            "building" : "1480",
            "coord" : [
                -73.9557413,
                40.7720266
            ]
        },
        "borough" : "Manhattan",
        "cuisine" : "Italian",
        "grades" : [
            {
                "date" : ISODate("1970-01-01T00:00:12.323Z"),
                "grade" : "A",
                "score" : 11
            },
            {
                "date" : ISODate("1970-01-01T00:02:01.212Z"),
                "grade" : "B",
                "score" : 17
            }
        ],
        "name" : "Vella",
        "restaurant_id" : "39"
    }

    */
    clock_t endTime = clock();
    std::cout << " insert total time: " << double(endTime - startTime) / CLOCKS_PER_SEC << " s" << std::endl;
    // @end: cpp-insert-a-document
}
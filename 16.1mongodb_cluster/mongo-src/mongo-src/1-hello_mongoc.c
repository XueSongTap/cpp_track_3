//hello_mongoc.c
 /*
 
 gcc -o 1-hello_mongoc 1-hello_mongoc.c \
    -I/usr/local/include/libbson-1.0 -I/usr/local/include/libmongoc-1.0 \
    -lmongoc-1.0 -lbson-1.0
 */
#include <mongoc/mongoc.h>
int main (int argc, char *argv[])
{
   const char *uri_string = "mongodb://localhost:27017";    // 连接默认的地址
   mongoc_uri_t *uri;	 // url
   mongoc_client_t *client;  // 客户端
   mongoc_database_t *database;   // 数据库
   mongoc_collection_t *collection;  // 集合
   bson_t *command, reply, *insert;
   bson_error_t error;
   char *str;
   bool retval;
 
   /*
    * Required to initialize libmongoc's internals 初始化内部
    */
   mongoc_init ();
 
   /*
    * Optionally get MongoDB URI from command line
    */
   if (argc > 1) {
      uri_string = argv[1];     // 可以指定其他mongodb服务器地址
   }
 
   /*
    * Safely create a MongoDB URI object from the given string
    */
   uri = mongoc_uri_new_with_error (uri_string, &error); // 连接MongoDB服务器
   if (!uri) {
      fprintf (stderr,
               "failed to parse URI: %s\n"
               "error message:       %s\n",
               uri_string,
               error.message);
      return EXIT_FAILURE;
   }
 
   /*
    * Create a new client instance，创建客户端实例
    */
   client = mongoc_client_new_from_uri (uri);
   if (!client) {
      return EXIT_FAILURE;
   }
 
   /*
    * Register the application name so we can track it in the profile logs
    * on the server. This can also be done from the URI (see other examples).
    */
   mongoc_client_set_appname (client, "connect-example");
 
   /*
    * Get a handle on the database "db_name" and collection "coll_name"
    */
   database = mongoc_client_get_database (client, "db_name_darren");   // 创建db  
   collection = mongoc_client_get_collection (client, "db_name_darren", "coll_name");// coll_name集合名
 
   /*
    * Do work. This example pings the database, prints the result as JSON and
    * performs an insert
    */
   command = BCON_NEW ("ping", BCON_INT32 (1));
 
   retval = mongoc_client_command_simple (
      client, "admin", command, NULL, &reply, &error);
 
   if (!retval) {
      fprintf (stderr, "%s\n", error.message);
      return EXIT_FAILURE;
   }
 
   str = bson_as_json (&reply, NULL);
   printf ("%s\n", str);  // { "ok" : 1.0 }
    // { "_id" : ObjectId("60db2849dc32a90f5263c7a2"), "hello" : "world" }
   insert = BCON_NEW ("name", BCON_UTF8 ("darren"));  // 组装一个json对象
    // 插入到集合
   if (!mongoc_collection_insert_one (collection, insert, NULL, NULL, &error)) {
      fprintf (stderr, "%s\n", error.message);
   }
 
   bson_destroy (insert);
   bson_destroy (&reply);
   bson_destroy (command);
   bson_free (str);
 
   /*
    * Release our handles and clean up libmongoc
    */
   mongoc_collection_destroy (collection);
   mongoc_database_destroy (database);
   mongoc_uri_destroy (uri);
   mongoc_client_destroy (client);
   mongoc_cleanup ();
 
   return EXIT_SUCCESS;
}

#include <iostream>
#include "servant/Communicator.h"
#include "Hello.h"

using namespace std;
using namespace TestApp;
using namespace tars;
static string helloObj = "TestApp.HelloServer.HelloObj";
int main(int argc,char ** argv)
{
    Communicator comm;

    try
    {
        //HelloPrx prx;
        //comm.stringToProxy("TestApp.HelloServer.HelloObj@tcp -h 192.168.31.248 -p 22785" , prx);
        //comm.stringToProxy("TestApp.HelloServer.HelloObj@tcp -h 192.168.206.128  -p 25769" , prx); 
		HelloPrx prx;
        comm.setProperty("locator", "tars.tarsregistry.QueryObj@tcp -h 192.168.206.128 -t 60000 -p 17890");
        prx = comm.stringToProxy<HelloPrx>(helloObj);        
		try
        {
            string sReq("hello world");
            string sRsp("");

            int iRet = prx->testHello(sReq, sRsp);
            cout<<"iRet:"<<iRet<<" sReq:"<<sReq<<" sRsp:"<<sRsp<<endl;

        }
        catch(exception &ex)
        {
            cerr << "ex:" << ex.what() << endl;
        }
        catch(...)
        {
            cerr << "unknown exception." << endl;
        }
    }
    catch(exception& e)
    {
        cerr << "exception:" << e.what() << endl;
    }
    catch (...)
    {
        cerr << "unknown exception." << endl;
    }

    return 0;
}

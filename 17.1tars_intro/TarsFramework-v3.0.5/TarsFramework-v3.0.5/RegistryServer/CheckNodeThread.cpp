﻿/**
 * Tencent is pleased to support the open source community by making Tars available.
 *
 * Copyright (C) 2016THL A29 Limited, a Tencent company. All rights reserved.
 *
 * Licensed under the BSD 3-Clause License (the "License"); you may not use this file except 
 * in compliance with the License. You may obtain a copy of the License at
 *
 * https://opensource.org/licenses/BSD-3-Clause
 *
 * Unless required by applicable law or agreed to in writing, software distributed 
 * under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR 
 * CONDITIONS OF ANY KIND, either express or implied. See the License for the 
 * specific language governing permissions and limitations under the License.
 */

#include "CheckNodeThread.h"

extern TC_Config * g_pconf;

CheckNodeThread::CheckNodeThread()
: _terminate(false)
, _nodeTimeout(60)
, _nodeTimeoutInterval(250)
{
}

CheckNodeThread::~CheckNodeThread()
{
    if (isAlive())
    {
        terminate();
        notifyAll();
        getThreadControl().join();
    }
}


int CheckNodeThread::init()
{
    TLOG_DEBUG("CheckNodeThread init"<<endl);

    //初始化配置db连接
    _db.init(g_pconf);

    //node心跳超时时间
    _nodeTimeout = TC_Common::strto<int>((*g_pconf).get("/tars/reap<nodeTimeout>", "250"));
    _nodeTimeout = _nodeTimeout < 15 ? 15 : _nodeTimeout;

    _nodeTimeoutInterval = TC_Common::strto<int>((*g_pconf).get("/tars/reap<nodeTimeoutInterval>", "60"));
    _nodeTimeoutInterval = _nodeTimeoutInterval < 15 ? 15 : _nodeTimeoutInterval;

    TLOG_DEBUG("CheckNodeThread init ok."<<endl);

    return 0;
}

void CheckNodeThread::terminate()
{
    TLOG_DEBUG("CheckNodeThread terminate" << endl);
    _terminate = true;
}

void CheckNodeThread::run()
{
    time_t tLastCheckNode = 0;
    time_t tNow;
    while(!_terminate)
    {
        try
        {
            tNow = TC_TimeProvider::getInstance()->getNow();
            //轮询心跳超时的node
            if(tNow - tLastCheckNode >= _nodeTimeoutInterval)
            {
			
                tLastCheckNode = tNow;
				
                _db.checkNodeTimeout(_nodeTimeout);

            }

            TC_ThreadLock::Lock lock(*this);
            timedWait(1000); //ms
        }
        catch(exception & ex)
        {
            TLOG_ERROR("CheckNodeThread exception:" << ex.what() << endl);
        }
        catch(...)
        {
            TLOG_ERROR("CheckNodeThread unknown exception" << endl);
        }
    }
}




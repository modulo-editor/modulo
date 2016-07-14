var searchIndex = {};
searchIndex["modulo"] = {"doc":"","items":[[0,"core","modulo","",null,null],[0,"core_msg","modulo::core","",null,null],[4,"ToCoreThreadMsg","modulo::core::core_msg","Messages that can be sent to the core thread.",null,null],[13,"FileThreadMsg","","Sends file thread message to the file thread matching the `FileThreadId`",0,null],[13,"SpawnFileThread","","Spawns a new file thread",0,null],[11,"fmt","","",0,null],[0,"core_thread","modulo::core","",null,null],[3,"Core","modulo::core::core_thread","",null,null],[11,"start","","",1,null],[11,"run","","Runs the event loop for the `CoreThread`",1,null],[11,"handle_spawn_file_thread","","",1,null],[0,"file","modulo","",null,null],[0,"file_msg","modulo::file","",null,null],[3,"FileThreadId","modulo::file::file_msg","",null,null],[12,"0","","",2,null],[4,"ToFileThreadMsg","","Messages that can be sent to the file thread to manipulate text",null,null],[13,"ReplaceText","","Replace text between the start point and end point with the data from the string.\nIf the end point is `None`, the text is just inserted at the start point.\nIf the string is the empty string, the text between the start point and end point is\ndeleted.",3,null],[13,"Save","","Saves the file to the OS file system. If the path is `None`, the file will not be saved.\nThe front end is responsible for making sure the file thread has a proper path to a file.\nThis may in the future accept a channel that",3,null],[4,"SaveResult","","Used to output the result of the save message. The frontend can use this information to know if\na file saved successfully.",null,null],[13,"Ok","","",4,null],[13,"Err","","",4,null],[13,"PromptForPath","","",4,null],[11,"clone","","",2,null],[11,"hash","","",2,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"fmt","","",2,null],[11,"fmt","","",3,null],[0,"file_thread","modulo::file","",null,null],[3,"FileThread","modulo::file::file_thread","A file thread represents one open file. It contains all the information about the data within\nthat file and listens for messages to manipulate the data within the file.",null,null],[11,"start","","Call to open a new file thread. The `path` parameter is the `Path` to the file to edit.\nIf `path` is `None`, an empty, untitled file is opened.\nIf the file at the path does not exist, the file is created when the file is saved.",5,{"inputs":[{"name":"filethreadid"},{"name":"option"},{"name":"sender"},{"name":"receiver"}],"output":null}],[11,"run","","Runs the event loop for the `FileThread`",5,null],[0,"text","modulo::file","",null,null],[3,"Line","modulo::file::text","Holds data for a single line of text.",null,null],[12,"text","","",6,null],[3,"Point","","Represents a cursor in text.",null,null],[12,"line","","",7,null],[12,"index","","",7,null],[11,"fmt","","",6,null],[11,"new","","",6,{"inputs":[{"name":"string"}],"output":{"name":"line"}}],[11,"fmt","","",7,null],[11,"new","","",7,{"inputs":[{"name":"usize"},{"name":"usize"}],"output":{"name":"point"}}]],"paths":[[4,"ToCoreThreadMsg"],[3,"Core"],[3,"FileThreadId"],[4,"ToFileThreadMsg"],[4,"SaveResult"],[3,"FileThread"],[3,"Line"],[3,"Point"]]};
searchIndex["log"] = {"doc":"A lightweight logging facade.","items":[[3,"LogRecord","log","The &quot;payload&quot; of a log message.",null,null],[3,"LogMetadata","","Metadata about a log message.",null,null],[3,"LogLocation","","The location of a log message.",null,null],[3,"MaxLogLevelFilter","","A token providing read and write access to the global maximum log level\nfilter.",null,null],[3,"SetLoggerError","","The type returned by `set_logger` if `set_logger` has already been called.",null,null],[3,"ShutdownLoggerError","","The type returned by `shutdown_logger_raw` if `shutdown_logger_raw` has\nalready been called or if `set_logger_raw` has not been called yet.",null,null],[4,"LogLevel","","An enum representing the available verbosity levels of the logging framework",null,null],[13,"Error","","The &quot;error&quot; level.",0,null],[13,"Warn","","The &quot;warn&quot; level.",0,null],[13,"Info","","The &quot;info&quot; level.",0,null],[13,"Debug","","The &quot;debug&quot; level.",0,null],[13,"Trace","","The &quot;trace&quot; level.",0,null],[4,"LogLevelFilter","","An enum representing the available verbosity level filters of the logging\nframework.",null,null],[13,"Off","","A level lower than all log levels.",1,null],[13,"Error","","Corresponds to the `Error` log level.",1,null],[13,"Warn","","Corresponds to the `Warn` log level.",1,null],[13,"Info","","Corresponds to the `Info` log level.",1,null],[13,"Debug","","Corresponds to the `Debug` log level.",1,null],[13,"Trace","","Corresponds to the `Trace` log level.",1,null],[5,"max_log_level","","Returns the current maximum log level.",null,{"inputs":[],"output":{"name":"loglevelfilter"}}],[5,"set_logger","","Sets the global logger.",null,{"inputs":[{"name":"m"}],"output":{"name":"result"}}],[5,"set_logger_raw","","Sets the global logger from a raw pointer.",null,{"inputs":[{"name":"m"}],"output":{"name":"result"}}],[5,"shutdown_logger","","Shuts down the global logger.",null,{"inputs":[],"output":{"name":"result"}}],[5,"shutdown_logger_raw","","Shuts down the global logger.",null,{"inputs":[],"output":{"name":"result"}}],[8,"Log","","A trait encapsulating the operations required of a logger",null,null],[10,"enabled","","Determines if a log message with the specified metadata would be\nlogged.",2,null],[10,"log","","Logs the `LogRecord`.",2,null],[11,"fmt","","",0,null],[11,"clone","","",0,null],[11,"eq","","",0,null],[11,"eq","","",0,null],[11,"partial_cmp","","",0,null],[11,"partial_cmp","","",0,null],[11,"cmp","","",0,null],[11,"from_str","","",0,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",0,null],[11,"max","","Returns the most verbose logging level.",0,{"inputs":[],"output":{"name":"loglevel"}}],[11,"to_log_level_filter","","Converts the `LogLevel` to the equivalent `LogLevelFilter`.",0,null],[11,"fmt","","",1,null],[11,"clone","","",1,null],[11,"eq","","",1,null],[11,"eq","","",1,null],[11,"partial_cmp","","",1,null],[11,"partial_cmp","","",1,null],[11,"cmp","","",1,null],[11,"from_str","","",1,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",1,null],[11,"max","","Returns the most verbose logging level filter.",1,{"inputs":[],"output":{"name":"loglevelfilter"}}],[11,"to_log_level","","Converts `self` to the equivalent `LogLevel`.",1,null],[11,"args","","The message body.",3,null],[11,"metadata","","Metadata about the log directive.",3,null],[11,"location","","The location of the log directive.",3,null],[11,"level","","The verbosity level of the message.",3,null],[11,"target","","The name of the target of the directive.",3,null],[11,"level","","The verbosity level of the message.",4,null],[11,"target","","The name of the target of the directive.",4,null],[11,"fmt","","",5,null],[11,"clone","","",5,null],[11,"module_path","","The module path of the message.",5,null],[11,"file","","The source file containing the message.",5,null],[11,"line","","The line containing the message.",5,null],[11,"fmt","","",6,null],[11,"get","","Gets the current maximum log level filter.",6,null],[11,"set","","Sets the maximum log level.",6,null],[11,"fmt","","",7,null],[11,"fmt","","",7,null],[11,"description","","",7,null],[11,"fmt","","",8,null],[11,"fmt","","",8,null],[11,"description","","",8,null],[14,"log!","","The standard logging macro.",null,null],[14,"error!","","Logs a message at the error level.",null,null],[14,"warn!","","Logs a message at the warn level.",null,null],[14,"info!","","Logs a message at the info level.",null,null],[14,"debug!","","Logs a message at the debug level.",null,null],[14,"trace!","","Logs a message at the trace level.",null,null],[14,"log_enabled!","","Determines if a message logged at the specified level in that module will\nbe logged.",null,null]],"paths":[[4,"LogLevel"],[4,"LogLevelFilter"],[8,"Log"],[3,"LogRecord"],[3,"LogMetadata"],[3,"LogLocation"],[3,"MaxLogLevelFilter"],[3,"SetLoggerError"],[3,"ShutdownLoggerError"]]};
initSearch(searchIndex);

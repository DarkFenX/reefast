import os
import pathlib
import re
import time
from collections import namedtuple
from enum import StrEnum, unique
from threading import Thread


class ParseError(Exception):
    pass


# pylint: disable=C0103
@unique
class Level(StrEnum):
    error = 'ERROR'
    warning = 'WARN'
    info = 'INFO'
    debug = 'DEBUG'
    trace = 'TRACE'


LogEntry = namedtuple('LogEntry', ('time', 'level', 'span', 'msg'))


class LogReader:

    __time = r'\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{3}'
    __level = '|'.join(Level)

    def __init__(self, path):
        self.__path = path
        self.__targets = []
        self.__execute_flag = False

    def add_target(self, target):
        self.__targets.append(target)

    def remove_target(self, target):
        self.__targets.remove(target)

    def run(self):
        self.__execute_flag = True
        t = Thread(target=self.__execute)
        t.start()

    def stop(self):
        self.__execute_flag = False

    def __follow(self):
        pathlib.Path(self.__path).touch(mode=0o644, exist_ok=True)
        with open(self.__path, 'r', encoding='utf-8') as f:
            f.seek(0, os.SEEK_END)
            while self.__execute_flag:
                line = f.readline()
                if not line:
                    time.sleep(0.1)
                    continue
                yield line

    def __parse(self, line):
        m = re.match(
            fr'^\[(?P<time>{self.__time})\]\s+'
            fr'(?P<level>{self.__level})\s'
            fr'((?P<span>\S+): )?'
            fr'(?P<msg>.*)\n$', line)
        if not m:
            raise ParseError(line)
        return LogEntry(time=m.group('time'), level=Level(m.group('level')), span=m.group('span'), msg=m.group('msg'))

    def __execute(self):
        for line in  self.__follow():
            # Should happen only if we were asked to stop following
            if line is None:
                return
            # Don't waste time on parsing when nobody is going to take it anyway
            if not self.__targets:
                continue
            try:
                entry = self.__parse(line)
            except ParseError as e:
                for target in self.__targets:
                    target.append_error(e)
                continue
            for target in self.__targets:
                target.append_entry(entry)


class LogCollector:

    def __init__(self):
        self.__buffer = []
        self.__errors = []

    def append_error(self, error):
        self.__errors.append(error)

    def append_entry(self, entry):
        self.__buffer.append(entry)

    @property
    def buffer(self):
        return self.__buffer

    @property
    def errors(self):
        return self.__errors

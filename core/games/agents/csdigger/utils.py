


# TODO: implement the k-map problem [[ find the simplest algebraic statement for a given function ]]
# TODO: Logic circuit simplification (SOP and POS) and its problems like XOR problem ___ http://www.32x8.com/index.html
# TODO: build a virtualenv like python's one has and compile all codes using pypy and cython in a virtualenv
# TODO: use multithreading to run It and Human code separately at the same time (itFollows algo)
# TODO: a new programming paradigm(using python C++ extension) to implement abstract concepts like itFollows and cancer
# TODO: change the address of object in python in run-time to build a virus like stux net algo to change its address constantly and bypass AVs
# TODO: understand all patterns(serach , sort and path finding) in graph , tree(BST and ...) , matrices DS and MDP env (A*, CSP, MDP, MCST..) and basic AI algos for agent creation with all graph formula
# TODO: binary prespective for solving problems using different data structures and base number and logical operators also think about maze algos and color handling in python
# TODO: dna computing using turing machine[encode/decode a dna] also create a woman using dna coding in python to combine ANN with dna
# TODO: try to extend algorithms using cython, ctypes, cffi or by writing an extension in C++ and rust - embedding python in C++ apps


import numpy as np
import os
import timeit
import random
import asyncio
from typing import Callable
import sys
import time
import math


print((lambda name, number: str(number)+name)(input("[+] Enter A Name : "), 456))

# =====================================
# overloading * operator
class int2(int):
    def __init__(self, x):
        # super(int2, self).__init__()
        self = x

    def __matmul__(self, i2):
        return self * i2 # i2 is int2 type


a = int2(2.3)
b = int2(3.5)
c = a @ b
print(c)
# -------------------
class int2():
    def __init__(self, x):
        self.value = x
        # self = x

    def __matmul__(self, i2):
        return int2(self.value * i2.value)


a = int2(2)
b = int2(3)
c = a @ b
print(c.value)

# =====================================

n = 5
A = np.diag(list(range(1, n+1)))

diag_B = np.diag(list(range(1, n)))
col_B = np.hstack((diag_B, np.zeros((diag_B.shape[0], 1), dtype=diag_B.dtype))) # add a new col of zeros at the end of mat
B = np.insert(col_B, 0, 0, axis=0) # insert zeros at first row

diag_C = np.diag(list(range(n-1, 0, -1)))
col_C = np.insert(diag_C, 0, 0, axis=1) # insert zeros at first col
C = np.insert(col_C, n-1, 0, axis=0) # insert zeros at the last row 

mat = np.add(np.add(A, B), C)

# =====================================

N = int(input("Enter a number "))
s = []
while N > 0:
    d = N//10
    r = N - 10*d
    s.append(r)
    N = d

d = 0
for i in range(len(s)):
    d += s[i]*math.pow(10,len(s)-i)/10

print(int(d))


# =====================================

class A():
    def __init__(self, a=None):
        self.a = a
        self.avar = "im a varible for A"
    
    def where(self):
        print("im inside {}".format(self.a))
        print(self.avar)
    


class B(A):
    def __init__(self, b=None):
        super().__init__("A")
        
    def edit(self):
        self.where()
        self.avar = "i edited a var ; now im a varible for B"
        print(self.avar)


if __name__ == "__main__":
    for i in range(4):
        b = B(b="{}".format(i))
        print(b)
        b.edit()

# -----------------------------

class A:
    def __init__(self, val):
        self.val = val
        print("[+] IM INSIDE A WIHT VALUE {}".format(self.val))
    
    def __a(self):
        class B(A):
            def __init__(self):
                print("[+] IM INSIDE B")
                super().__init__(67)
        return B()
    
    def b(self):
        self.__a()

if __name__ == "__main__":
    a = A(3)
    print(a.b())


# =====================================
# calculate all possible subset for a set
# base prespective for n nested loop and other dynamic problems!
# for i in [0..2]:
#     for j in [0..2]:
#         for p in [0..2]:
#             for q in [0..2]:

# 0 0 0 0 -> 0 0 0 2 -> 0 0 1 0 -> 0 0 1 2
# 0000, 0001, 0002, 0010, 0011, 0012, 0020, ..., 2222, (1)0000
# i j p q ...n ta...
# i1 i2 i3 ... in

# setlst = [[0,1,2], [0,1,2], [0,1,2]]
setlst = [["cs","wo","force", "shi"], ["cs","wo","force", "shi"], ["cs","wo","force", "shi"]]
n = len(setlst)
k = len(setlst[0])
idx = 0
lst = []

# METHOD 1
def select():
    global setlst, lst, idx
    lst = []
    t = idx
    for i in range(n):
        lst.append(setlst[i][t%k])
        t = t//k
    idx += 1
    return idx != n**k+1
while select():
    print(lst, "=>" , idx, ''.join(str(i) for i in lst))

# METHOD 2
i = [0 for _ in range(0,n)]
def finished():
    global i
    for j in i:
        if j is not 0:
            return False
    return True
def check():
    global i, k, n
    for j in range(len(i)):
        if i[j] >= k:
            i[j] = 0
            if j+1 < n:
                i[j+1] += 1
def inc():
    global i
    i[0] += 1
    check()

def select():
    global setlst, i, n, k
    tmp = []
    for j in range(len(i)):
        yield setlst[j][i[j]]

print([m for m in select()])
inc()
while not finished():
    print([m for m in select()])
    inc()

# METHOD 3
def com(lst):
    N = len(lst)
    for i in range(2**N):
        combo = []
        for j in range(N):
            if (i >> j) % 2 == 1:
                combo.append(lst[j])
                print(lst[j])
        yield combo

for i in com([1,2,3,5]):
    print(i, ", ", end="")

# METHOD 4 : TODO: fix the bugs and make it faster!!!!!! use multi processing using some AI algo
import copy
lst = [ [1,2,3], [4,5,6], [7,8,9] ]
y, combo, lt = [], [], copy.deepcopy(lst)
def firstElem(lst):
    if len(lst)>0:
        lst.remove(lst[0])
        return lst
    else:
        return
def allCombo(lst):
    global y, combo
    if len(lst)>0:
        a = lst[0][0]
        while a is not None:
            y.append(a)
            allCombo(firstElem(lst))
            lt[0].remove(a)
            a = lt[0][0]
        lt.remove(lt[0])
        allCombo(lt)
    else:
        combo.append(y)
    return combo
print(allCombo(lst))

# =====================================

# >>> d={"adventurous":"aventurero","bold":"audaz","courageous":"valiente"} 
# >>>d.items()
# [('courageous', 'valiente'), ('adventurous', 'aventurero'), ('bold', 'audaz')]
# >>> d.keys()
# ['courageous', 'adventurous', 'bold']
# >>> d.values()
# ['valiente', 'aventurero', 'audaz']
# >>> my_list = [('a', 1), ('b', 2)]
# >>> dict(my_list)
# {'a': 1, 'b': 2}


# TODO: runtime and time complexity issue!!! make the code simple!

def makelst(A):
    avglist = []
    for key in A.keys():
        avglist.append(float(A[key]["avg"]))
    return avglist

def sortavglst(avglist):
    n = len(avglist)
    for i in range(n):
        for j in range(0, n-i-1):
            if avglist[j] > avglist[j+1] :
                avglist[j], avglist[j+1] = avglist[j+1], avglist[j]
    return avglist
    

def createRATE(avgsortedlst, A):
    for key in A.keys():
        for i in range(len(avgsortedlst)):
            if A[key]["avg"] == avgsortedlst[i]:
                A[key]["rate"] = i+1

def calAVG(A):
    for key in A.keys():
        s = 0
        for i in A[key]["course_info"]:
            s+= float(i[1])
            avg = s/len(A[key]["course_info"])
            A[key]["avg"] = avg

def fillMe():
    A = {}
    for i in range(int(input("[+] REGISTERING FOR ? >>> "))):
        A[int(input("[+] STU ID >>> "))] = {"name": str(input("[+] NAME >>> ")), 
           "lname": str(input("[+] LASTNAME >>> ")),
           "avg": None,
           "rate": None,
           "course_info": [tuple(input("[+] WRITE COURSE NAME->MARK >>> ").split("->")) for cname in range(int(input("[+] COURSE NUMBER ? >>> ")))]
           }

    return A

def printMe(A):
    print("\n\n")
    for key in A.keys():
        print("STUID =========\n")
        print("{}\n".format(A[key]))
        print("FIRSTNAME ========\n")
        print("{}\n".format(A[key]["name"]))
        print("LASTNAME ========\n")
        print("{}\n".format(A[key]["lname"]))
        print("AVERAGE ========\n")
        print("{}\n".format(A[key]["avg"]))
        print("RATE ========\n")
        print("{}\n".format(A[key]["rate"]))
        print("COURSE INFO =========\n")
        for i in A[key]["course_info"]:
            print("{} -> {}".format(i[0], i[1]))
        print("\n")

if __name__ =="__main__":
    A = fillMe()
    calAVG(A)
    createRATE(sortavglst(makelst(A)), A)
    printMe(A)

# =====================================
# multi same dict key and their values

# METHOD 1
class Dictlist(dict):
    def __setitem__(self, key, value):
        try:
            self[key]
        except KeyError:
            super(Dictlist, self).__setitem__(key, [])
        self[key].append(value)

d = dictlist.Dictlist()
d['test'] = 1
d['test'] = 2
d['test'] = 3
# >>> d
# {'test': [1, 2, 3]}
d['other'] = 100
# >>> d
# {'test': [1, 2, 3], 'other': [100]}

# METHOD 2
class DictList(dict):
    def __setitem__(self, key, value):
        try:
            # Assumes there is a list on the key
            self[key].append(value) 
        except KeyError: # if fails because there is no key
            super(DictList, self).__setitem__(key, value)
        except AttributeError: # if fails because it is not a list
            super(DictList, self).__setitem__(key, [self[key], value])

dl = DictList()
dl['a'] = 1
dl['b'] = 2
dl['b'] = 3


# OUTPUT: {'a': 1, 'b': [2, 3]}

        
# =====================================

# topol algo

tsbst: list = [None]

def subset(sett: list) -> list:
        biN:Callable[str, str] = lambda el : ''.join(reversed([str((el>>i) & 1) for i in range(len(sett))]))
        tpx:int = 2**len(sett)
        for i in range(1,tpx):
                # breakpoint()
                # print(list(biN(i%tpx)))
                yield [i for i,j in zip(sett, list(biN(i%tpx))) if int(bool(i)) is int(j)]


# subset((lambda x : [input() for n in range(int(input("[+] NUMBER OF MEMBER SET : ")))])([]))
for sbst in subset(sys.argv[1].split(",")):
        tsbst.append(set(sbst))


smallestTopol: set = [None, tsbst[len(tsbst)-1]]
largestTopol: list = tsbst

print(f"\n[+] THE LARGEST TOPOLOGY IS\n\n\t {largestTopol}\n")
print(f"[+] THE SMALLEST TOPOLOGY IS\n\n\t {smallestTopol}\n")


# =====================================

# itFollows - black scary python virus

import time

class Human():
    def __init__(self):
        pass
    def __haveSex(self):
        pass
    def __isAlive(self):
        pass
    def __transpose(self):
        pass
    def __isFollow(self):
        pass

class It(Human):
    def __init__(self):
        pass
    def __walking(self):
        pass
    def __changeAvatar(self):
        pass


# =====================================

# python script to link the some urls into their related files

# we have a file called urls.txt and ex: script_1.py , script_2.py , script_3.py
# we want to make a link between urls exist in urls.txt and its relates script
# NOTE: this process is constantly running when a new script comes to the directory
# NOTE: this process will split the every script into two element => [0] : script , [1] : urls
# NOTE: this process will put all urls([1]) in urls.txt file and make a link to its related script([0])
# TODO: use multithreading for this purpose  




# -------------------------------------------------------------------------------------------
# harmonic calculator
from math import log
from timeit import *

def hur(n):
    if n is 1:
        return 1
    else:
        return hur(n-1) + 1/n
print(hur(3))

def H(n):
    gamma = 0.57721566490153286060651209008240243104215933593992
    print(gamma + log(n) + 0.5/n - 1./(12*n**2) + 1./(120*n**4))

H(20)

# =======================
# prime detection
def detectP(n):
    primes = []
    for possiblePrime in range(2, n+1):
        isPrime = True
        for x in range(2, int(possiblePrime ** 0.5) + 1):
            if possiblePrime % x == 0:
                isPrime = False
                break
        if isPrime:
            primes.append(possiblePrime)
    return primes

print(detectP(10))
print(timeit('detectP(34)', globals=globals(), number=10000))


# -------------------------------------------------------------------------------------------
# designing pattern on a simple OOP problem
# employees insured and salary

employees = []
#   ======================================

#                 CLASSES

import os
class Employee():
  
  def __init__(self, pcode):
    self.name = ""
    self.lname = ""
    self.meli_code = 0
    self.pcode = pcode
    self.fixed_sal = 0.0
    self.hours_of_work = 0
    self.tax = None
    self.insurance = 0.0
    self.overtime= 0.0
    self.final_sal = 0.0
  
  
  def status(self):
    if self.hours_of_work > 40:
      hours_of_overtime = self.hours_of_work - 40
      self.overtime = hours_of_overtime * 30000
    self.insurance = self.fixed_sal * 0.05
    
    if self.fixed_sal < 4000000:
      self.tax = "moaf"
    if 4000000 < self.fixed_sal < 5000000:
      self.tax = (self.fixed_sal - 4000000) * 0.10
    if 5000000 < self.fixed_sal < 7000000:
      self.tax = (self.fixed_sal - 4000000) * 0.15
    if self.fixed_sal > 7000000:
      self.tax = (self.fixed_sal - 4000000) * 0.20
      
    self.final_sal = (self.overtime + self.fixed_sal) - (self.insurance + self.tax)
  
  def addinfo(self, name, lname, meli_code, fixed_sal, how):
    self.name = name
    self.lname = lname
    self.meli_code = meli_code
    self.fixed_sal = fixed_sal
    self.hours_of_work = how
  
  def showinfo(self):
    print("\n==============\nEMPLOYEE INFO\n==============\n")
    print("[+] NAME          : ", self.name)
    print("[+] LAST NAME     : ", self.lname)
    print("[+] MELI CODE     : ", self.meli_code)
    print("[+] PERSONAL CODE : ", self.pcode)
    print("[+] FIXED SALARY  : ", self.fixed_sal)
    print("[+] HOURS OF WORK : ", self.hours_of_work)
    print("[+] TAX           : ", self.tax)
    print("[+] INSURANCE     : ", self.insurance)
    print("[+] OVERTIME      : ", self.overtime)
    print("[+] FINAL SALARY  : ", self.final_sal)
  
  
#   ==================================

#           FUNCTIONS   

def showallempinfo():
  for i in range(len(employees)):
    employees[i].showinfo()
    print("\t********")

    
def findbypcode():
#   os.system("cls")
  while True:
    pcode = input("[+] Enter personal code to find an employee >> ")
    if not pcode:
      break
    for i in range(len(employees)):
      if employees[i].pcode == int(pcode):
        print("[+] FOUND ONE MATCH ... ")
        employees[i].showinfo()
        break
      else:
        print("[+] NOTHING FOUND... ")
        break
      
def savetofile(emps, filename):
  with open(filename+".txt", "w") as f:
    for i in range(len(emps)):
      f.write("\n==============\nEMPLOYEE INFO\n==============\n")
      f.write("[+] NAME          : ")
      f.write(str(employees[i].name)+"\n")
      f.write("[+] LAST NAME     : ")
      f.write(str(employees[i].lname)+"\n")
      f.write("[+] MELI CODE     : ")
      f.write(str(employees[i].meli_code)+"\n")
      f.write("[+] PERSONAL CODE : ")
      f.write(str(employees[i].pcode)+"\n")
      f.write("[+] FIXED SALARY  : ")
      f.write(str(employees[i].fixed_sal)+"\n")
      f.write("[+] HOURS OF WORK : ")
      f.write(str(employees[i].hours_of_work)+"\n")
      f.write("[+] TAX           : ")
      f.write(str(employees[i].tax)+"\n")
      f.write("[+] INSURANCE     : ")
      f.write(str(employees[i].insurance)+"\n")
      f.write("[+] OVERTIME      : ")
      f.write(str(employees[i].overtime)+"\n")
      f.write("[+] FINAL SALARY  : ")
      f.write(str(employees[i].final_sal)+"\n")
      
def readfromfile(path):
  with open(path+".txt", "r") as f:
    for line in f:
      print(line)
    
  
  
def save3emptofile():
  data = []
  for i in range(3):
    code = input(f"\tEnter meli code or personal code of person {i+1}, ba comma joda konid, like : 00198222, 33 >> ")
    if "," in code:
      meli_code = code.split(",")[0]
      pcode = code.split(",")[1]
      for j in range(len(employees)):
        if employees[j].meli_code == meli_code and employees[j].pcode == pcode:
          data.append(employees[j])
    else:
      for j in range(len(employees)):
        if employees[j].meli_code == code or employees[j].pcode == code:
          data.append(employees[j])
  
  savetofile(data, "data")
      
      

#   ==============================

#             MAIN


if __name__ == "__main__":
  
#   initalizing....
  for i in range(int(input("[+] Employee.N >> "))):
    name = str(input("\tEnter name >> "))
    lname = str(input("\tEnter last name >> "))
    meli_code = int(input("\tEnter meli code >> "))
    fixed_sal = float(input("\tEnter fixed salary >> "))
    how = int(input("\tEnter hours of work >> "))
    e = Employee(i+1)
    e.addinfo(name, lname, meli_code, fixed_sal, how)
    e.status()
    employees.append(e)
    if i > 1 :
      print("\t@@@@@@@@@@@@@@@@@@@")
    
  
  showallempinfo()
  print("[[[[[[ FINDING EMPLOYEE ]]]]]]")
  findbypcode()
  print("[[[[[[ SAVING TO  FILE .... ]]]]]]")
  savetofile(employees, "all_employees")
  print("[[[[[[ READING FROM FILE ]]]]]]")
  readfromfile("all_employees")
  print("[[[[[[ SAVING 3 EMPLOYEE TO FILE ]]]]]]")
  save3emptofile()

    
# -------------------------------------------------------------------------------------------




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
from typing import Callable, List
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

# convert 1D list into 2D list

def reshape(arr, m, n):
    if len(arr) % m != 0 and len(arr) % n != 0:
        return False
    mat = []
    for row in range(m):
        lst = []
        for mult in range(row*n, (row+1)*n):
            lst.append(arr[mult])
        mat.append(lst)
    print(mat)


arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
reshape(arr, 3, 5)

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

# ACTIVITY PROBLEMS

# given a set of n activities with their start and finish times, 
# we need to select maximum number of non-conflicting activities 
# that can be performed by a single person, 
# given that the person can handle only one activity at a time.

activities = [(1, 3), (3, 4), (2, 5), (0, 7), (5, 9), (8, 10), (11, 12)]


# find overlapping activities
def find_none_overlapping(activities):
    none_overlapping_activities = []
    # we sort the activities based on their finishing times, unlike the starting time
    # that we had to wait for the longest finishing time to be completed
    # thus we couldn't choose other activities due to handling only
    # one at a time and conflicting issues!
    sorted_activities_based_on_fi = sorted(activities, key=lambda acts: acts[1])
    # select the first least finishing time activity as the starting one
    # this intuition let us greedily choosing the maximum activities
    least_act = activities[0]
    for i in range(1, len(activities)):
        # if the starting time of other activity is greater than the 
        # finishing time of the first one we know that we can do that 
        # activity after finishing the first one cause when we finish 
        # the first one the second on will start due to having greater starting time! 
        if activities[i][0] >= least_act[1]:
            none_overlapping_activities.append(activities[i])
    return none_overlapping_activities

print(find_none_overlapping(activities))

# -------------------------------------------------------------------------------------------

# longest substring without repeating characters

class lols:
    def __init__(self, string : str):
        self.string = string
    def solve(self) -> int:
        if len(self.string) == 0:
            return 0
        if len({self.string[i] : i for i in range(len(self.string))}) == 1:
            return 1
        sub_s = []
        ls = self.string[0]
        last_char = self.string[0]
        for c in range(1, len(self.string)):
            if self.string[c] != last_char:
                ls+=self.string[c]
            elif self.string[c] == last_char:
                ls = self.string[c]
            if len(ls) > 1:
                sub_s.append(ls)
            last_char = self.string[c]
        last_sub = sub_s[len(sub_s)-1]
        dict_sub_s = {last_sub[i] : i for i in range(len(last_sub))}
        return len(dict_sub_s)

                            
user_input = input("Enter String >>> ")
assert 0 <= len(user_input) <= 5 * 104
p = lols(user_input)
n_sub_str = p.solve()
print(n_sub_str)


# -------------------------------------------------------------------------------------------

# unhappy friends
# https://leetcode.com/contest/weekly-contest-206/problems/count-unhappy-friends/
# eg ::: input: [[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]], pairs = [[1, 3], [0, 2]], output: 4

if __name__ == "__main__":
    f_n = int(input("Enter friends ::: "))
    if 2 <= f_n <= 500 and f_n % 2 == 0:
        preferences = []
        pairs       = []
        for i in range(f_n):
            friends = []
            for j in range(1, f_n):
                f = int(input(f"Enter friends of person {i} based on preferences order ::: "))
                if f not in friends and 0 <= f <= f_n-1 and f != i:
                    friends.append(f)
                else:
                    print("[-] either repeated friend in rel list, lower/upper bound issue or found existing person her/him-self in his/her rel list")
                    sys.exit(1)
            preferences.append(friends)

        for k in range(int(f_n/2)):
            p_friends = []
            for l in range(2):
                f = int(input(f"Enter person {l} of pair {k} ::: "))
                if f not in p_friends and 0 <= f <= f_n-1:
                    p_friends.append(f)
                else:
                    print("[-] either repeated friend in one pair or lower/upper bound issue")
                    sys.exit(1)
            pairs.append(p_friends)

        p_tmp = pairs[0]
        for n in range(1, len(pairs)):
            if pairs[n][0] == p_tmp[0] or pairs[n][1] == p_tmp[1]:
                print("[-] found existing person in another pair")
                sys.exit(1)
            else:
                p_tmp = pairs[n]

        count_u_h_f = 0
        for p in pairs:
            f_p = p[0]
            s_p = p[1]
            pref_f_p = preferences[f_p]
            pref_s_p = preferences[s_p]
            is_f_happy = True
            is_s_happy = True
            if pref_f_p[0] != s_p:
                is_f_happy = False
                count_u_h_f += 1
            if pref_s_p[0] != f_p:
                is_s_happy = False
                count_u_h_f += 1

        
        if count_u_h_f != 0:
            print(f"{count_u_h_f} friends are unhappy :-(")
        else:
            print("no one is unhappy :-)")


# -------------------------------------------------------------------------------------------

# check string is transformable or not

def detect_digit(s):
    for c in range(len(s)):
        if s[c] not in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']:
            return False
        else:
            return True

s = input("first string ::: ")
t = input("second string ::: ")

assert 1 <= len(s) <= 105, "too long"
assert len(s) == len(t), "length must be equal"
assert len(s) != 1 or len(t) != 1, "can't transform"
assert detect_digit(s) or detect_digit(t), "chars must be digit"

def is_transformable(s):
    list_string, list_t, sorted_s_lst = list(s), list(t), list(s)
    for i in range(2, len(list_string), 2): # start from 2 beacuse of the upper bound of slicing string 
        cutted_string = sorted_s_lst[:i] # i doesn't count
        sorted_cutted_string = [str(n) for n in sorted(list(filter(lambda n: int(n), cutted_string)))]
        sorted_s_lst = sorted_cutted_string+sorted_s_lst[i:]
    unequal_indices = [sorted_s_lst.index(i) for i, j in zip(sorted_s_lst, list_t) if i!=j]
    if sorted_s_lst == list_string:
        return False
    else:
        greater_val = sorted_s_lst[unequal_indices[0]]
        smaller_val = sorted_s_lst[unequal_indices[1]]
        sorted_s_lst[unequal_indices[1]] = greater_val
        sorted_s_lst[unequal_indices[0]] = smaller_val
        sorted_s = "".join(sorted_s_lst)
        if sorted_s == t:
            return True


if is_transformable(s):
    print("|>> is transformable <<|")
else:
    print("|>> is NOT transformable <<|")

# -------------------------------------------------------------------------------------------

# PATTERN SEARCHING (REGX)

# help : https://www.geeksforgeeks.org/finite-automata-algorithm-for-pattern-searching/

class FA:
    def __init__(self) -> None:
        self.__state_char_mat = None
        self.__pattern = ""

    def compute(self, pattern: str) -> None:
        self.__pattern = pattern
        '''
            pattern: ACACAGA
            Number of states in FA will be M+1 where M is length of the pattern. The main thing to construct FA 
            is to get the next state from the current state for every possible character. Given a character x and a state k, 
            we can get the next state by considering the string “pat[0..k-1]x” which is basically concatenation of pattern 
            characters pat[0], pat[1] … pat[k-1] and the character x. The idea is to get length of the longest prefix of 
            the given pattern such that the prefix is also suffix of “pat[0..k-1]x”. The value of length gives us the next state. 
            For example, let us see how to get the next state from current state 5 and character ‘C’ in the above diagram. 
            We need to consider the string, “pat[0..4]C” which is “ACACAC”. The length of the longest prefix of the pattern 
            such that the prefix is suffix of “ACACAC”is 4 (“ACAC”). So the next state (from state 5) is 4 for character ‘C’. 
        '''
        number_of_states = len(pattern) + 1
        cols =  {pattern[i]: 0 for i in range(len(pattern))}
        code_ascii = [ord(c) for c in cols.keys()]
        self.__state_char_mat = [[0 for i in range(len(code_ascii))] for _ in range(number_of_states)]
        for state in range(number_of_states):
            for x in range(len(code_ascii)):
                next_state = self.__next_state(state, code_ascii[x])
                self.__state_char_mat[state][x] = next_state

    def __next_state(self, state, x):
        '''
            there are “len(pattern) + 1” states and when
            we move from state k with char x the next state
            will be one of the remaining states except the one 
            that has accepted x which we have ”pat[0..k-1]” states
            and the -1 is because of x after that which is a 
            final state that accept our pattern.
            so to calculate the next state we have to find
            length of the longest prefix of “pat[0..k-1]x” 
            such that the prefix is also suffix of “pat[0..k-1]x”.
            trying all possible prefixes starting from the 
            longest possible that can be a suffix of “pat[0..k-1]x”.
        '''

        # if the char x is same as next char in pattern[state], then simply increment state  
        if state < len(self.__pattern) and x == ord(self.__pattern[state]):
            return state + 1
        i = 0
        for next_state in range(state, 0, -1): # 0 to k 
            if ord(self.__pattern[next_state-1]) == x:
                while(i<next_state-1): # not next_state-1 because self.__pattern[next_state-1] is x and we want to check every chars before x 
                    if self.__pattern[i] != self.__pattern[state-next_state+1+i]: 
                        break
                    i+=1
                if i == next_state-1: # we found prefix which is also a suffix
                    return next_state  
        return 0


    def accept(self, string: str) -> bool:
        '''
            build a FA machine which accept the pattern then 
            we pass our string through our machine if it reach
            the final state at any position meaning that we found our pattern
            at that position.
        '''
        state = 0
        for i in range(len(string)):
            state = self.__state_char_mat[state][ord(string[i])]
            if state == len(self.__pattern):
                print(f"[+] pattern found at index: {i-len(self.__pattern)+1}")
                return True
            else:
                return False


class Regx(FA):
    def __init__(self, _input: str) -> None:
        self.__input = _input
        self.compute(_input)

    def match(self, pattern) -> bool:
        not_in_string = []
        built_pattern = []
        code_ascii  = [ord(c) for c in self.__input]
        ascii_count = {ord(c): self.__input.count(c) for c in self.__input}
        
        if len(pattern) < len(self.__input):
            return False
        else:
            for i in range(len(pattern)):
                if ord(pattern[i]) in ascii_count:
                    built_pattern.append(pattern[i])
                elif ord(pattern[i]) not in ascii_count and i+1 < len(pattern) and pattern[i] != "*" and pattern[i+1] == "*":
                    built_pattern.append(pattern[i])
                elif pattern[i] == "*":
                    if i == 0:
                        break
                    star_no_rep = ascii_count[ord(built_pattern[i-1])] if ord(built_pattern[i-1]) in ascii_count else 0
                    if star_no_rep == 0:
                        built_pattern.append("0")
                    else:
                        while built_pattern.count(built_pattern[i-1]) != star_no_rep:
                            built_pattern.append(built_pattern[i-1])
                elif pattern[i] == ".":
                    built_pattern.append(self.__input[i])
        
        built_pattern = [ord(c) for c in built_pattern if ord(c) in ascii_count and c != "0"]
        if built_pattern == code_ascii:
            return True
        else:
            return False



_input  = input("Enter string ::: ")
pattern = input("Enter pattern ::: ")

reg = Regx(_input)
if not reg.match(pattern):
    print("[-] pattern doesn't match the entire string")
else:
    print("[+] pattern match with the entire string")

# -------------------------------------------------------------------------------------------

# STATUS : incomplete

# https://codeforces.com/problemset/problem/1374/C


min_ops = 0
open_stack = []

def move_ith(s, index):
    global min_ops
    if s[index] == ")": # couldn't find the openning bracket for that
        s = s[index : ] + s[ : index] 
        index = is_reg(s)
        if index == -1:
            return -1
        else:
            move_ith(s, index)

    elif s[index] == "(": # couldn't find the closing bracket for that
        pass
    else:
        pass


def is_reg(s):
    global open_stack
    for i in range(len(s)):
        if s[i] == "(":
            if i == len(s)-1:
                return i
            else:
                open_stack.append(s[i])
        if s[i] == ")":
            if i == 0:
                return 0
            if open_stack:
                if open_stack[len(open_stack)-1] == "(":
                    open_stack.pop()
            else:
                return i
    return -1


test_cases, i = int(input("test cases >>> ")), 0
while i < test_cases < 2000:
    _input = input()
    s = _input.replace(" ", "")
    if 2 <= len(s) <= 50:
        if len(s) % 2 == 0:
            index = is_reg(s)
            if index != -1:
                move_ith(s, index)
            else:
                print("no need to make it reg")
        else:
            print("the length must be even!")
            break
    print("=============")
    i+=1

# -------------------------------------------------------------------------------------------

# STATUS : incomplete

# https://codeforces.com/problemset/problem/1373/G


rows, k, m, min_changes, cols = 5, 3, 5, 0, 5

class List(list):
    def __getitem__(self, index):
        if type(index) == int and index > 0:
           index -= 1
        if type(index) == slice:
           start, stop = index.start, index.stop
           if start and start > 0:
              start -= 1
           if stop and stop > 0:
              stop -=  1
           index = slice(start, stop, index.step)
        return super().__getitem__(index)
    def __setitem__(self, index, val):
        super().__setitem__(index-1, val)
        

CHESSBOARD = List(List([ '__' for _ in range(1, rows+1)]) for _ in range(1, rows+1))
inputs = [(4, 4), (3, 5), (2, 4), (3, 4), (3, 5)]

for points in inputs:
    if CHESSBOARD[points[1]][points[0]] != '_p_':
        CHESSBOARD[points[1]][points[0]] = '_p_'


def add_row():
    global min_changes, rows
    CHESSBOARD.insert(1, ['__' for i in range(1, cols+1)])
    min_changes += 1
    rows += 1


def move_pawn(point):
    
    pawn_added = False
    col = point[0]
    row = point[1]


    print(f"trying to move pawn from cell ({col},{row})")
    if row+1 <= rows:
        if col == k:                
            if CHESSBOARD[row+1][col] != '_p_':
                print(f"into cell ({col}, {row+1})")
                CHESSBOARD[row+1][col] = '_p_'
                CHESSBOARD[row][col] = '__'
            if CHESSBOARD[row+1][col] == '_p_':
                print(f"can't put in cell ({col},{row+1}) there is already a pawn in cell ({col}, {row+1})")
                CHESSBOARD[row][col] = '__'

        elif col-1 == k:
            if CHESSBOARD[row+1][col-1] != '_p_':
                print(f"putting pawn in cell ({col-1}, {row+1})")
                CHESSBOARD[row+1][col-1] = '_p_'
                CHESSBOARD[row][col] = '__'
            if CHESSBOARD[row+1][col-1] == '_p_':
                print(f"can't put in cell ({col-1},{row+1}) there is already a pawn in cell ({col-1}, {row+1})")
                CHESSBOARD[row][col] = '__'

        elif col+1 == k:
            if CHESSBOARD[row+1][col+1] != '_p_':
                print(f"putting pawn in cell ({col+1}, {row+1})")
                CHESSBOARD[row+1][col+1] = '_p_'
                CHESSBOARD[row][col] = '__'
            if CHESSBOARD[row+1][col+1] == '_p_':
                print(f"can't put in cell ({col+1},{row+1}) there is already a pawn in cell ({col+1}, {row+1})")
                CHESSBOARD[row][col] = '__'

    else:
        print("adding row")
        add_row()
        move_pawn(point)



if __name__ == "__main__":
    for i in range(m):
        move_pawn(inputs[i])
        print(min_changes)
        min_changes = 0

    print(CHESSBOARD)


# -------------------------------------------------------------------------------------------

# STATUS : buggy

# https://codeforces.com/problemset/problem/1375/G

def n_count_degree(nodes):
    nodes = [node[0] for node in nodes] + [node[1] for node in nodes]
    n_rep_node = {n: 0 for n in nodes}
    for node in nodes:
        if node in n_rep_node:
            n_rep_node[node]+=1
    return n_rep_node


class Node:
    def __init__(self, data):
        self.data: int = data
        self.children: List[Node] = []

    def add_child(self, node):
        self.children.append(node)

class Tree:
    def __init__(self, n, v_u):
        assert 3 <= n <= 2e+5
        assert 1 <= len(v_u) < n
        self.vertices    = n
        self.v_u         = v_u
        self.min_ops     = 0

        for i in range(len(v_u)):
            if v_u[i][0] == v_u[i][1]:
                print("same node in one tuple!")
                sys.exit(1)
            if 1 >= v_u[i][0] >= self.vertices:
                print("greater than node detected!")
                sys.exit(1)

        self.build_tree()
        self.show()


    def build_tree(self):

        self.tree = {Node(data): n_count_degree(self.v_u)[data] for data in n_count_degree(self.v_u)} # the value is the degree of each node

        for i in range(len(self.v_u)):
            for node in self.tree:
                if self.v_u[i][0] == node.data:
                    child = self.v_u[i][1]
                    node.add_child(Node(child))
                elif self.v_u[i][1] == node.data:
                    child = self.v_u[i][0]
                    node.add_child(Node(child))


    def is_star(self):
        n_rep_node = n_count_degree(self.v_u)
        rep_nodes = len(list(filter(lambda rep: True if rep>=2 else False, n_rep_node.values())))
        if rep_nodes >= 2: # more than 2 nodes are repeated
            return False
        elif rep_nodes == 1: # only one node is repeated
            return True
        else:
            print("Something went wrong!")
            sys.exit(1)


    def show(self):
        print("\n---------SHOWING TREE STRUCTURE---------\n")
        for node in self.tree:
            print(node.data,"____")
            if node.children:
                for child in node.children:
                    print("\t____",child.data)


    def build_star(self):
        print("\n---------BUILDING STAR FROM TREE---------\n")
        '''
        **BUGGY**
            1 - Choose three vertices a, b, and c such that b is adjacent to both a and c.
            2 - For every vertex d other than b that is adjacent to a, remove the edge connecting d and a and add the edge connecting d and c.
            3 - Delete the edge connecting a and b and add the edge connecting a and c.
        '''
        for node in self.tree:
            if len(node.children) == 2:
                a = node.children[0]
                b = node
                c = node.children[1]
                print(f"choosing a = {a.data}, b = {b.data} and {c.data}.")
                if len(a.children) >= 1:
                    for child in a.children:
                        d = child
                        c.add_child(d)
                        a.children.remove(d)
                    b.children.remove(a)
                if len(c.children) >= 1:
                    for child in c.children:
                        d = child
                        a.add_child(d)
                        c.children.remove(d)
                    b.children.remove(c)
                self.min_ops += 1

        self.show()



if __name__ == "__main__":

    n = input("nodes >> ")
    print("edges [separated by space]: \n")
    v_u = list(map(lambda node: tuple((int(node[0]), int(node[1]))), [tuple(input().split(" ")) for _ in range(int(n)-1)]))


    t0 = Tree(int(n), v_u)
    if t0.is_star():
        print("tree is already a star!")
    else:
        t0.build_star()
        print(f"minimum operations >>> {t0.min_ops}")
#include <Windows.h>
#include <iostream>
#include <string>

using namespace std;

// method that imports a resource from a file and returns it's raw contents as a string
string importResource(string resourceType, string resourceName) {

  LPCWSTR resourceTypeW = (LPCWSTR)resourceType.c_str();
  LPCWSTR resourceNameW = (LPCWSTR)resourceName.c_str();
  HRSRC hResource = FindResource(NULL, resourceNameW, resourceTypeW);
  if (hResource == NULL) {
    cout << "[!] FindResource failed: " << GetLastError() << endl;
    return "";
  }
  
  DWORD dwSize = SizeofResource(NULL, hResource);
  if (dwSize == 0) {
    cout << "[!] SizeofResource failed: " << GetLastError() << endl;
    return "";
  }
  
  HGLOBAL hGlobal = LoadResource(NULL, hResource);
  if (hGlobal == NULL) {
    cout << "[!] LoadResource failed: " << GetLastError() << endl;
    return "";
  }
  
  LPVOID lpResource = LockResource(hGlobal);
  if (lpResource == NULL) {
    cout << "[!] LockResource failed: " << GetLastError() << endl;
    return "";
  }
  
  string resourceContents((char*)lpResource, dwSize);
  return resourceContents;
}
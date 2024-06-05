#include <Windows.h>
#include <string>
#include <iostream>
#include <cstdio>

using namespace std;

string obfuscateIPv4(string payload) {
  // add padding to the payload
  int padding = 4 - (payload.size() % 4);
  for (int i = 0; i < padding; i++) {
    payload += '\0';
  }
  int output;
  string obfuscatedResult = "";
  for(int c=0, counter=0, i=0; i<payload.length(); i+=4, c=0) {
    while(c<4){
      output = payload[i+c];
      obfuscatedResult += to_string(output);
      if(c<3) {
        obfuscatedResult += '.';
      }
      c++;
    }
    obfuscatedResult += ' ';
  }
  return obfuscatedResult;
}

string deobfuscateIPv4Detected(string payload) { // very crude - will be detected
  string deobfuscatedResult = "";
  string temp = "";
  for(int i=0; i<payload.length(); i++) {
    if(payload[i] == ' ' || i == payload.length()-1) {
      if(i == payload.length()-1) {
        temp += payload[i];
      }
      int num = stoi(temp);
      deobfuscatedResult += (char)num;
      temp = "";
    } else if(payload[i] == '.') {
      int num = stoi(temp);
      deobfuscatedResult += (char)num;
      temp = "";
    } else {
      temp += payload[i];
    }
  }
  return deobfuscatedResult;
}

void obfuscatePayloadIPv4() {
  string payload = "";
  cout << "[i] Base payload: "<< payload << endl;
  string obfuscated = obfuscateIPv4(payload);
  cout << "[i] Obfuscated payload: " << obfuscated << endl;
  string deobfuscated = deobfuscateIPv4Detected(obfuscated);
  cout << "[i] Detectable deobfuscated payload: " << deobfuscated << endl;

  cout << "[i] Press any key to continue..." << endl;
  cin.get();
}

string obfuscateIPv6(string payload) {
  // add padding to the payload
  int padding = 16 - (payload.size() % 16);
  for (int i = 0; i < padding; i++) {
    payload += '\0';
  }
  int output;
  string obfuscatedResult = "";
  for(int c=0, counter=0, i=0; i<payload.length(); i+=16, c=0) {
    int temp[16];
    while(c<16){
      output = payload[i+c];
      temp[c++] = output;
    }
    for(int j=0; j<16; j+=2) {
      string hex = "";
      sprintf((char*)hex.c_str(), "%02X%02X", temp[j], temp[j+1]);
      obfuscatedResult += hex;
      if(j<14) {
        obfuscatedResult += ':';
      }
    }
    obfuscatedResult += ' ';
  }
  return obfuscatedResult;
}

string deobfuscateIPv6Detected(string payload) { // very crude - will be detected
  string deobfuscatedResult = "";
  string temp = "";
  for(int i=0; i<payload.length(); i++) {
    if(payload[i] == ' ' || i == payload.length()-1) {
      if(i == payload.length()-1) {
        temp += payload[i];
      }
      int num = stoi(temp, 0, 16);
      deobfuscatedResult += (char)num;
      temp = "";
    } else if(payload[i] == ':') {
      int num = stoi(temp, 0, 16);
      deobfuscatedResult += (char)num;
      temp = "";
    } else {
      temp += payload[i];
    }
  }
  return deobfuscatedResult;
}

void obfuscatePayloadIPv6() {
  string payload = "";
  cout << "[i] Base payload: "<< payload << endl;
  string obfuscated = obfuscateIPv6(payload);
  cout << "[i] Obfuscated payload: " << obfuscated << endl;
  string deobfuscated = deobfuscateIPv6Detected(obfuscated);
  cout << "[i] Detectable deobfuscated payload: " << deobfuscated << endl;

  cout << "[i] Press any key to continue..." << endl;
  cin.get();
}
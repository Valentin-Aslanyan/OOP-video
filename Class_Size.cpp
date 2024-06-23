#include <iostream>
using namespace std;


class Alice{
public:
	int x;
	char y;
	
	Alice(){
		x=0;
		y='a';
	}
};


int main(){
	Alice Bob;

	int x=0;
	char y='a';

	cout << "Class: " << sizeof(Bob) << endl; //8
	cout << "Separate: " << sizeof(x)+sizeof(y) << endl; //5

	return 0;
}

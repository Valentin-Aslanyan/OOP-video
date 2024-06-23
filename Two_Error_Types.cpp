#include <iostream>
using namespace std;


class Charlie{
private:
	int z;
public:
	int x;
	char y;
	
	Charlie(){
		x=0;
		y='a';
		z=0;
	}
	
	int get_z(){
		return z;
	}
	void set_z(int a){
		z=a;
	}
};


int main(){

	int k;
	Charlie Dave;

	k = 5000000000; //Capability error
	Dave.z = 6; //Organization error

	return 0;
}

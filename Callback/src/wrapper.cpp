#include "./wrapper.h"

#include <stdlib.h>
#include <iostream>

using namespace std;

extern "C" {
	void create_array(unsigned int* arr, unsigned int num)
	{
		if(arr == NULL)
			return;

		/* Memory preallocated */
		for(int i = 0; i < num; i++)
			arr[i] = (i+5)*(i+5);
	}
}


import * as React from 'react';
import Main_page from './Main_page.js';
import Scan_barcode from './Scan_barcode.js';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';

const Stack = createNativeStackNavigator();

const productFromBackend = ({route, navigation}) => {
  alert('dupa');
  return (
  <Image 
    source={{
      uri: '192.168.243.204:8000/images/' + route.navigation.state.params.img_url}}
    style={{width: 100, height: 100}}
  />
  );
};

export default function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator
        screenOptions={{
          headerShown: false
        }}
        initialRouteName= 'Main_page' 
        >
        <Stack.Screen 
          name="Main_page"
          component={Main_page}
          options={{}}
        />
        <Stack.Screen 
          name="Scan_barcode"
          component={Scan_barcode}
          options={{}}
        />
         <Stack.Screen 
          name="productFromBackend"
          component={productFromBackend}
          options={{}}
        />
        
      </Stack.Navigator>
    </NavigationContainer>
  );
}
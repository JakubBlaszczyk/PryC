import * as React from 'react';
import Main_page from './Main_page.js';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';

const Stack = createNativeStackNavigator();

export default function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator
        screenOptions={{
          headerShown: false
        }}>
        <Stack.Screen 
          name="MainPage"
          
          component={Main_page}
          options={{}}
        />
      </Stack.Navigator>
    </NavigationContainer>
  );
}
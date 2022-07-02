--
-- Simple example to send follwing set of RADIUS requests:
--
--  * RADIUS Access-Request
--  * RADIUS Accounting-Start
--  * sleep for 5 seconds
--  * RADIUS Accounting-Stop
--

-- Import libwl lib to acccess whirl API
local libwl = require('libwl')

--
-- Define workload with configuration of a test
--
-- Following configuration items:
--
--  * workers - Number of threads to use
--  * radius_servers - List of authorization and accounting RADIUS servers
--  * imsi_range = Range of IMSI(s) that will be used on per-user basis
--
workload = {
   workers = 4,
   radius_servers = {
      auth = '127.0.0.1:1812',
      acct = '127.0.0.1:1813'
   },
   imsi_range = '262019876543210-262019876543310'
}

-- Entry point
function run(state)
   -- Build RADIUS Access-Requst packet
   state = radius_auth(state)

   -- Send RADIUS Access-Request packet
   state = libwl.radius_send(workload['radius_servers']['auth'], state['packet'], 'secret')

   -- Build RADIUS Accounting-Start packet
   state = radius_acct_start(state)

   -- Send RADIUS Accounting-Request Start packet
   state = libwl.radius_send(workload['radius_servers']['acct'], state['packet'], 'secret')

   -- Build RADIUS Accounting-Request Stop packet
   state = radius_acct_start(state)

   -- Send RADIUS Accounting-Request Stop packet
   state = libwl.radius_send(workload['radius_servers']['acct'], state['packet'], 'secret')

   return
end

-- Build RADIUS Access-Request packet
function radius_auth(state)
   -- Initialize empty RADIUS packet
   packet = {}

   -- Fill RADIUS packet with attributes
   packet['3GPP-IMSI'] = state['3GPP-IMSI']
   packet['3GPP-RAT-Type'] = 6
   packet['User-Name'] = 'username' .. '@' .. 'nas-id'
   packet['NAS-IP-Address'] = '127.0.0.1'
   packet['3GPP-SGSN-MCC-MNC'] = '40101'
   packet['Service-Type'] = 2
   packet['Called-Station-Id'] = '00-10-A4-23-19-C0'
   packet['Calling-Station-Id'] = '00-10-A4-23-19-C0'
   packet['3GPP-IMEISV'] = '35-209900-176148-87'
   packet['3GPP-SGSN-Address'] = '127.0.0.2'

   -- RADIUS response callback
   packet['on_response'] = function(state)
      -- Try to get Framed-IP-Address from response
      ip = state['response']['Framed-IP-Address']

      -- Check if we have Framed-IP-Address within response put
      -- into state
      if ip ~= nil then
         state['Framed-IP-Address'] = ip
      else
         libwl.stop_with_error()
      end
   end

   -- put packet into the state machine
   state['packet'] = packet

   -- return the state so it could be used to send RADIUS request
   return state
end

-- Build RADIUS Accounting-Start packet
function radius_acct_start(state)
   -- Initialize empty RADIUS packet
   packet = {}

   -- Fill RADIUS packet with attributes
   packet['Acct-Status-Type'] = 1
   packet['Framed-IP-Protocol'] = 2
   packet['3GPP-IMSI'] = state['3GPP-IMSI']
   packet['Framed-IP-Address'] = state['Framed-IP-Address']
   packet['Acct-Session-Id'] = 'session-' .. '-' .. state['3GPP-IMSI']

   -- put packet into the state machine
   state['packet'] = packet

   -- return the state so it could be used to send RADIUS request
   return state
end

-- Build RADIUS Accounting-Stop packet
function radius_acct_stop(state)
   -- Initialize empty RADIUS packet
   packet = {}

   -- Fill RADIUS packet with attributes
   packet['Acct-Status-Type'] = 2
   packet['Framed-IP-Protocol'] = 2
   packet['3GPP-IMSI'] = state['3GPP-IMSI']
   packet['Framed-IP-Address'] = state['Framed-IP-Address']
   packet['Acct-Session-Id'] = 'session-' .. '-' .. state['3GPP-IMSI']

   -- put packet into the state machine
   state['packet'] = packet

   -- return the state so it could be used to send RADIUS request
   return state
end
